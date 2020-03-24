use crate::score::*;
use crate::{RectF, remove_element};
use std::convert::{TryInto, TryFrom};

/// One row of measures for all instruments;
/// a complete piece of the timeline
/// TODO: I think we dont need this to be a element stored in rc
/// use it as a struct instead. DOnt ref to it in the measure. If we assume there is one system
#[derive(Debug, Clone)]
pub struct System {
	element: ElementData,
	measures: Vec<MeasureRef>,
	staves: Vec<SysStaff>,
	spanner_segments: Vec<SpannerSegmentRef>,

	/// left margin for instrument name, brackets etc.
	left_margin: f32,
	fixed_down_distance: bool,

	// TODO
}

impl System {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		/// TODO: how about a bintree map? Need to update keys on change though
		/// Helps probably a lot more since there is a lot more lookup than midway update
		measures: vec![],
		staves: vec![],
		spanner_segments: vec![],
		left_margin: 0.0,
		fixed_down_distance: false
	})}
}

impl Element for System {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::System }
}

impl System {
	pub fn measure_count(&self) -> usize { self.measures.len() }

	/// Appends measure to the system
	pub fn append_measure(&mut self, m: MeasureRef) {
		// TODO: mayby give measure a weakref to the system?
		self.measures.push(m);
		// TODO: calculate time first?
	}

	/// Removes matching measure from the system
	pub fn remove_measure(&mut self, m: &MeasureRef) {
		remove_element(&mut self.measures, m);
	}

	pub fn remove_last_measure(&mut self) {
		if !self.measures.is_empty() {
			self.measures.pop();
		}
	}
}

impl System {
	/// Finds first chord which overlaps with the current time at the given track
	pub fn find_chordrest(&self, mut time: Fraction, track: Track) -> Option<ChordRef> {
		let measure: El<Measure> = self.find_measure(time)?.try_into().ok()?;
		measure.with(|measure| {
			if measure.is_mm_rest()  {
				time = measure.time();
			}

			// TODO: can probably add a filter map and custom funtional thingie (for now while debugging not)
			let mut last = measure.segments().first();
			for (_, segment) in measure.segments().iter_ty(Fraction::zero(), SegmentTypeMask::CHORDREST) {
				if segment.borrow_el().time() > time { break }
				if let Some(el) = segment.borrow_el().element(track) {
					if let SegmentRef::Rest(rest) = el {
						if rest.borrow_el().gap() { continue }
					}
					last = Some(segment);
				}
			}

			let el = last?.clone().borrow_el().element(track)?.clone();
			if let SegmentRef::Rest(rest) = &el {
				if rest.borrow_el().gap() { return None }
			}
			return ChordRef::try_from(el).ok();
		})
	}

	/// Finds first measure which overlaps with the given time
	pub fn find_measure(&self, time: Fraction) -> Option<MeasureRef> {
		if time == Fraction::new(-1, 1) {
			return self.measures.last().cloned();
		} else if time <= Fraction::new(0, 1) {
			return self.measures.first().cloned();
		}

		let mut last: Option<&MeasureRef> = None;
		for m in self.measures.iter() {
			if time < m.as_trait().time() {
				return last.cloned();
			}
			last = Some(m);
		}

		// Check last measure
		if let Some(last) = last {
			if time >= last.as_trait().time() && time <= last.as_trait().end_time() {
				return Some(last.clone())
			}
		}

		return None;
	}

	/// Finds the first measure which overlaps with the given time
	/// If the measure is a part of a multimeasure rest. Then the first measure is returned
	pub fn find_measure_mm(&self, time: Fraction) -> Option<MeasureRef> {
		let measure = self.find_measure(time)?;
		if let MeasureRef::Measure(measure) = &measure {
			if self.style().value_bool(StyleName::CreateMultiMeasureRests) {
				if let Some(mm_rest) = measure.borrow_el().mm_rest() {
					return Some(mm_rest);
				}
			}
		}
		return Some(measure);
	}
}

#[derive(Debug, Clone)]
pub struct SysStaff {
	bbox: RectF,
}

impl Default for SysStaff {
	fn default() -> Self { Self {
		bbox: Default::default()
	}}
}


#[cfg(test)]
mod tests {
	use crate::testing;
	use crate::score::*;

	#[test]
	fn test_add_measures() {
		let score = testing::setup_score();
		let part = Part::new(score.clone(), "Triangle".to_string());
		let staff = Staff::new(score.clone());
		score.insert_part(part.clone(), 0);
		score.insert_staff(staff.clone(), &part, 0);

		let system = score.system();

		for i in 0..10 {
			let measure = Measure::new(score.clone()).with_mut_i(|mut measure| {
				measure.set_time(Fraction::new(i * 4, 4));
			});
			system.borrow_mut_el().append_measure(measure.clone().into());

			let chord = Chord::new(score.clone()).with_mut_i(|mut chord| {
				chord.set_duration_type(Duration::new(DurationType::Half, 0));
			});
			Measure::add_at(measure.clone(), chord.into(), Fraction::new(0, 4));
		}

		assert_eq!(system.borrow_el().measure_count(), 10);
		for i in 0..10 {
			let measure = system.borrow_el().find_measure(Fraction::new(i * 4, 4));
			assert!(measure.is_some());
			assert_eq!(measure.unwrap().as_trait().time(), Fraction::new(i * 4, 4));

			let measure = system.borrow_el().find_measure(Fraction::new(i * 4 + 1, 4));
			assert!(measure.is_some());
			assert_eq!(measure.unwrap().as_trait().time(), Fraction::new(i * 4, 4));

			let chord = system.borrow_el().find_chordrest(Fraction::new(i * 4, 4), 0);
			assert!(chord.is_some());
			assert_eq!(chord.unwrap().as_trait().time(), Fraction::new(i * 4, 4));
		}
	}
}