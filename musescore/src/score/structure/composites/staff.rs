use crate::score::*;
use crate::Color;

/// Global staff data not directly related to renderer.
#[derive(Debug, Clone)]
pub struct Staff {
	element: ElementData,

	/// Part the staff element belongs to
	part: Option<ElWeak<Part>>,

	/// List of Clefs indexed using Ticks
	clefs: ClefTypeList,
	default_clef_type: ClefTypeGroup,

	/// List of Keys indexed using Ticks
	keys: KeyList,
	key_default: KeySigEvent,
	/// List of TimeSignatures indexed using Ticks
	timesigs: TimesigList,

	/// span barline to next staff
	bar_line_span: i32,
	/// line of start staff to draw the barline from (0 = staff top line, ...)
	bar_line_from: i32,
	/// line of end staff to draw the bar line to (0= staff bottom line, ...)
	bar_line_to: i32,

	invisible: bool,

	/// user edited extra distance
	user_dist: f32,

	color: Color,

	/// List of Staff Types indexed using Ticks
	staff_type_list: StaffTypeList,
	staff_type_default: StaffType,
}

impl Staff {
	pub fn new(score: Score) -> El<Self> {
		new_element(Self {
			element: ElementData::new(score),
			part: None,
			clefs: ClefTypeList::new(),
			default_clef_type: Default::default(),
			keys: (KeyList::new()),
			key_default: Default::default(),
			timesigs: TimesigList::new(),
			bar_line_span: 0,
			bar_line_from: 0,
			bar_line_to: 0,
			invisible: false,
			user_dist: 0.0,
			color: [0, 0, 0, 255],
			staff_type_list: StaffTypeList::new(),
			staff_type_default: Default::default()
		})
	}

	pub fn part(&self) -> &Option<ElWeak<Part>> { &self.part }
	pub fn set_part(&mut self, v: Option<ElWeak<Part>>) { self.part = v }

	pub fn clefs(&self) -> &ClefTypeList { &self.clefs }
	pub fn set_clefs(&mut self, v: ClefTypeList) { self.clefs = v }

	pub fn default_clef_type(&self) -> &ClefTypeGroup { &self.default_clef_type }
	pub fn set_default_clef_type(&mut self, v: ClefTypeGroup) { self.default_clef_type = v }

	pub fn keys(&self) -> &KeyList { &self.keys }
	//pub fn set_keys(&mut self, v: KeyList) { self.keys = v }

	pub fn timesigs(&self) -> &TimesigList { &self.timesigs }
	//pub fn set_timesigs(&mut self, v: TimesigList) { self.timesigs = v }

	/// TODO: ask part instead
	pub fn show(&self) -> bool { true }

	pub fn bar_line_span(&self) -> i32 { self.bar_line_span }
	pub fn set_bar_line_span(&mut self, v: i32) { self.bar_line_span = v }
	pub fn bar_line_from(&self) -> i32 { self.bar_line_from }
	pub fn set_bar_line_from(&mut self, v: i32) { self.bar_line_from = v }
	pub fn bar_line_to(&self) -> i32 { self.bar_line_to }
	pub fn set_bar_line_to(&mut self, v: i32) { self.bar_line_to = v }

	pub fn invisible(&self) -> bool { self.invisible }
	pub fn set_invisible(&mut self, v: bool) { self.invisible = v }
	pub fn user_dist(&self) -> f32 { self.user_dist }
	pub fn set_user_dist(&mut self, v: f32) { self.user_dist = v }

	pub fn timesig(&self, time: &Fraction) -> Option<&El<TimeSig>> { self.timesigs.get(time.ticks()) }
	pub fn timestretch(&self, tick: &Fraction) -> Fraction {
		if let Some(timesig) = self.timesig(tick) { timesig.borrow_el().stretch().clone() } else { Fraction::new(1, 1) }
	}

	pub fn color(&self) -> &Color { &self.color }
	pub fn set_color(&mut self, v: Color) { self.color = v }

	pub fn lines(&self, tick: &Fraction) -> Line { self.staff_type(tick).lines() }
	pub fn set_lines(&mut self, tick: &Fraction, v: Line) { self.staff_type_mut(tick).set_lines(v) }
	pub fn line_distance(&self, tick: &Fraction) -> Spatium { self.staff_type(tick).line_distance() }

	pub fn spatium(&self, tick: &Fraction) -> f32 {
		self.score().spatium() * self.mag(tick)
	}
	pub fn mag(&self, tick: &Fraction) -> f32 {
		(if self.small(tick) { self.style().value_f32(StyleName::SmallStaffMag) } else { 1.0 }) * self.user_mag(tick)
	}
	pub fn user_mag(&self, tick: &Fraction) -> f32 { self.staff_type(tick).user_mag() }
	pub fn set_user_mag(&mut self, tick: &Fraction, v: f32) { self.staff_type_mut(tick).set_user_mag(v) }
	pub fn small(&self, tick: &Fraction) -> bool { self.staff_type(tick).small() }
	pub fn set_small(&mut self, tick: &Fraction, v: bool) { self.staff_type_mut(tick).set_small(v) }

	pub fn staff_type(&self, tick: &Fraction) -> &StaffType {
		self.staff_type_list.get(tick.ticks()).unwrap_or(&self.staff_type_default)
	}
	pub fn staff_type_mut(&mut self, tick: &Fraction) -> &mut StaffType {
		self.staff_type_list.get_mut(tick.ticks()).unwrap_or(&mut self.staff_type_default)
	}
	pub fn set_staff_type(&mut self, tick: &Fraction, v: StaffType) {
		self.staff_type_list.set(tick.ticks(), v)
	}

	pub fn keysig_event(&self, tick: &Fraction) -> &KeySigEvent {
		self.keys.get(tick.ticks()).unwrap_or(&self.key_default)
	}
	pub fn keysig_event_mut(&mut self, tick: &Fraction) -> &mut KeySigEvent {
		self.keys.get_mut(tick.ticks()).unwrap_or(&mut self.key_default)
	}
}

impl Staff {
	pub fn group(&self, time: &Fraction) -> Groups {
		// TODO: mayby not store group in timesig. Then we can pass reference
		if let Some(ts) = self.timesig(time) {
			ts.with(|ts| {
				if !ts.groups().is_empty() { ts.groups().clone() }
				else { Groups::endings(ts.sig()).clone() }
			})
		} else {
			if let Some(MeasureRef::Measure(m)) = self.score().find_measure(*time) {
				Groups::endings(m.borrow_el().timesig()).clone()
			} else {
				Groups::endings(&Fraction::new(4, 4)).clone()
			}
		}
	}

	pub fn key(&self, tick: &Fraction) -> Key { self.keysig_event(tick).key() }
	pub fn set_keysig(&mut self, time: &Fraction, v: KeySigEvent) {
		self.keys.set(time.ticks(), v)
	}

	pub fn set_clef(&mut self, e: El<Clef>) {
		if e.borrow_el().generated() { return; }
		let track = e.borrow_el().track();

		e.with(|segemnt| {

		});

		if let Some(segment) = e.borrow_el().segment() {
			let time = segment.borrow_el().time();
			if let Some(measure) = segment.borrow_el().measure() {
				for s in measure.borrow_el().segment_next_iter(time) {
					let s = s.borrow_el();
					if s.time() != time { break; }
					if s.is_clef() || s.is_header_clef() {
						if let Some(other) = s.element(track) {
							// adding this clef has no effect on the clefs list
							if other.as_trait().generated() { return; }
						}
					}
				}
			}
		}

		self.clefs.set(e.borrow_el().time().ticks(), e.borrow_el().clef_type_group().clone())
	}
	pub fn remove_clef(&mut self, e: El<Clef>) {
		if e.borrow_el().generated() { return; }
		let track = e.borrow_el().track();

		e.borrow_el().segment().with(|segment| {
			let time = segment.time();
			if let Some(measure) = segment.measure() {
				for s in measure.borrow_el().segment_next_iter(time) {
					let s = s.borrow_el();
					if s.time() != time { break; }
					if s.is_clef() || s.is_header_clef() {
						if let Some(other) = s.element(track) {
							// removing this clef has no effect on the clefs list
							if other.as_trait().generated() { return; } // TODO: warning
						}
					}
				}
			}

			self.clefs.remove(time.ticks());
			if let Some(measure) = segment.measure() {
				for s in measure.borrow_el().segment_prev_iter(time) {
					let s = s.borrow_el();
					if s.time() != time { break; }
					if s.is_clef() || s.is_header_clef() {
						if let Some(SegmentRef::Clef(other)) = s.element(track) {
							// a previous clef at the same tick position gets valid
							if other.borrow_el().generated() {
								self.clefs.set(time.ticks(), other.borrow_el().clef_type_group().clone())
							}
						}
					}
				}
			}
		});
	}

	pub fn add_timesig(&mut self, timesig: El<TimeSig>) {
		if let Some(segment) = timesig.borrow_el().segment() {
			if segment.borrow_el().is_timesig() {
				self.timesigs.set(segment.borrow_el().time().ticks(), timesig.clone());
			}
		}
	}
	pub fn remove_timesig(&mut self, timesig: El<TimeSig>) {
		if let Some(segment) = timesig.borrow_el().segment() {
			if segment.borrow_el().is_timesig() {
				self.timesigs.remove(segment.borrow_el().time().ticks());
			}
		}
	}
}

impl Element for Staff {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Staff }

	fn staff(&self) -> Option<El<Staff>> { self.get_ref_ty() }
	fn part(&self) -> Option<El<Part>> { self.part.as_ref().and_then(ElWeak::upgrade) }
}

pub type ClefTypeList = OrderedCollecton<ClefTypeGroup>;
pub type KeyList = OrderedCollecton<KeySigEvent>;
pub type TimesigList = OrderedCollecton<El<TimeSig>>;
pub type StaffTypeList = OrderedCollecton<StaffType>;

#[cfg(test)]
mod tests {
	use crate::testing;
	//use crate::score::*;

	#[test]
	fn test_add_measure() {
		let _score = testing::setup_score();
		// TODO: add measure tests
	}
}