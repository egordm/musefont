use crate::*;
use crate::score::*;
use std::convert::{TryFrom, TryInto};

/// # Segment
/// A segment holds all vertical aligned staff elements.
/// Segments are typed and contain only Elements of the same type.
///
/// All Elements in a segment start at the same tick. The Segment can store one Element for each
/// voice in each staff in the score.
/// Some elements (Clef, KeySig, TimeSig etc.) are assumed to always have voice zero and can be found in elist[staffIdx * VOICES];
///
/// Segments are children of Measures and store Clefs, KeySigs, TimeSigs, BarLines and ChordRests.
#[derive(Debug, Clone)]
pub struct Segment {
	element: ElementData,

	segment_type: SegmentTypeMask,
	/// midi tick position (read only)
	rel_time: Fraction,
	duration: Fraction,
	extra_leading_space: Spatium,
	stretch: bool,

	/// The list of annotations (read only)
	annotations: Vec<ElementRef>,
	/// Element storage, size = [staves * VOICES]
	elist: Vec<Option<SegmentRef>>,
	// size = staves
	//shapes: Vec<Shape>,
	/// size = staves
	dot_pos_x: Vec<f32>,
}

impl Segment {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		segment_type: SegmentTypeMask::INVALID,
		rel_time: Fraction::new(0, 1),
		duration: Fraction::new(0, 1),
		extra_leading_space: Spatium(0.0),
		stretch: false,
		annotations: vec![],
		elist: vec![None], // Default to one staff
		dot_pos_x: vec![]
	})}

	pub fn segment_type(&self) -> SegmentTypeMask { self.segment_type }
	pub fn set_segment_type(&mut self, v: impl Into<SegmentTypeMask> + Copy) { self.segment_type = v.into() }

	pub fn rel_time(&self) -> Fraction { self.rel_time }
	pub fn set_rel_time(&mut self, t: Fraction) { self.rel_time = t }
	pub fn duration(&self) -> Fraction { self.duration }
	pub fn set_duration(&mut self, v: Fraction) { self.duration = v }

	pub fn extra_leading_space(&self) -> Spatium { self.extra_leading_space }
	pub fn set_extra_leading_space(&mut self, v: Spatium) { self.extra_leading_space = v }
	pub fn stretch(&self) -> bool { self.stretch }
	pub fn set_stretch(&mut self, v: bool) { self.stretch = v }

	pub fn annotations(&self) -> &Vec<ElementRef> { &self.annotations }
	pub fn set_annotations(&mut self, v: Vec<ElementRef>) { self.annotations = v }

	pub fn dot_pos_x(&self, staff_id: StaffId) -> f32 { self.dot_pos_x[staff_id as usize] }
	pub fn set_dot_pos_x(&mut self, staff_id: StaffId, v: f32) { self.dot_pos_x[staff_id as usize] = v }

	pub fn elements(&self) -> &Vec<Option<SegmentRef>> { &self.elist }
	pub fn element(&self, track: Track) -> Option<&SegmentRef> { self.elist.get(track as usize)?.as_ref() }
	pub fn set_element(&mut self, track: Track, e: Option<SegmentRef>) { self.elist[track as usize] = e }

	pub fn next(&self) -> Option<El<Segment>> {
		self.measure()?.borrow_el().segment_next_iter(self.time()).skip(1).next().cloned()
	}
	pub fn prev(&self) -> Option<El<Segment>> {
		self.measure()?.borrow_el().segment_prev_iter(self.time()).skip(1).next().cloned()
	}
	pub fn next_type(&self, t: impl Into<SegmentTypeMask> + Copy) -> Option<El<Segment>> {
		self.measure()?.borrow_el().segment_next_iter(self.time()).skip(1)
			.filter(|e| e.borrow_el().is_type(t)).next().cloned()
	}
	pub fn prev_type(&self, t: impl Into<SegmentTypeMask> + Copy) -> Option<El<Segment>> {
		self.measure()?.borrow_el().segment_prev_iter(self.time()).skip(1)
			.filter(|e| e.borrow_el().is_type(t)).next().cloned()
	}

	pub fn next_chordrest(&self, track: Track, backwards: bool) -> Option<ChordRef> {
		let f = |segment: &El<Segment>| -> Result<ChordRef, ()> {
			if let Some(e) = segment.borrow_el().element(track) {
				if e.get_type() != SegmentType::Chord && e.get_type() != SegmentType::Rest {
					return ChordRef::try_from(e.clone())
				}
			}
			return Err(());
		};

		if backwards {
			for segment in self.measure()?.borrow_el().segment_prev_iter(self.time()) {
				if let Ok(res) = f(segment) { return ChordRef::try_from(res).ok() }
			}
		} else {
			for segment in self.measure()?.borrow_el().segment_next_iter(self.time()) {
				if let Ok(res) = f(segment) { return ChordRef::try_from(res).ok() }
			}
		}
		return None;
	}

	pub fn is_type(&self, t: impl Into<SegmentTypeMask>) -> bool { let t = t.into(); self.segment_type & t == t }
	pub fn is_begin_barline(&self) -> bool { self.is_type(SegmentTypeMask::BEGIN_BARLINE) }
	pub fn is_start_repeat_barline(&self) -> bool { self.is_type(SegmentTypeMask::START_REPEAT_BARLINE) }
	pub fn is_barline(&self) -> bool { self.is_type(SegmentTypeMask::BARLINE) }
	pub fn is_end_barline(&self) -> bool { self.is_type(SegmentTypeMask::END_BARLINE) }
	pub fn is_clef(&self) -> bool { self.is_type(SegmentTypeMask::CLEF) }
	pub fn is_header_clef(&self) -> bool { self.is_type(SegmentTypeMask::HEADER_CLEF) }
	pub fn is_keysig(&self) -> bool { self.is_type(SegmentTypeMask::KEYSIG) }
	pub fn is_ambitus(&self) -> bool { self.is_type(SegmentTypeMask::AMBITUS) }
	pub fn is_timesig(&self) -> bool { self.is_type(SegmentTypeMask::TIMESIG) }
	pub fn is_breath(&self) -> bool { self.is_type(SegmentTypeMask::BREATH) }
	pub fn is_chordrest(&self) -> bool { self.is_type(SegmentTypeMask::CHORDREST) }
	pub fn is_keysig_announce(&self) -> bool { self.is_type(SegmentTypeMask::KEYSIG_ANNOUNCE) }
	pub fn is_timesig_announce(&self) -> bool { self.is_type(SegmentTypeMask::TIMESIG_ANNOUNCE) }

	/// Adds the given element to the chord
	pub fn add(e: El<Self>, c: SegmentRef) {
		let track = e.borrow_el().track();
		c.as_trait_mut().attach(e.borrow_el().get_ref(), track);

		match c {
			SegmentRef::Clef(_) | SegmentRef::TimeSig(_) | SegmentRef::KeySig(_) => {
				Self::add_element(e.clone(), c.clone());
				e.with(|er| {
					if !er.generated() {
						er.staff().with_mut(|mut staff| {
							match &c {
								SegmentRef::Clef(c) => staff.set_clef(c.clone()),
								SegmentRef::TimeSig(c) => staff.add_timesig(c.clone()),
								SegmentRef::KeySig(c) => staff.set_keysig(&er.time(), c.borrow_el().sig().clone()),
								_ => unreachable!()
							}
						});
					}
				});
			},
			SegmentRef::Chord(_) | SegmentRef::Rest(_) => Self::add_chordrest(e, c.try_into().unwrap()),
			SegmentRef::Barline(_) => Self::add_element(e, c.into()),
		}

	}

	fn add_chordrest(e: El<Self>, c: ChordRef) {
		let track = e.borrow_el().track();
		if track % constants::VOICES as Track > 0 {
			let visible = match &c {
				ChordRef::Chord(c) => {
					c.borrow_el().notes().iter().any(|n| n.borrow_el().visible())
				}
				ChordRef::Rest(c) => {
					c.borrow_el().visible()
				}
			};
			e.borrow_mut_el().set_visible(visible);
		}

		Self::add_element(e, c.into())
	}

	fn add_element(e: El<Self>, c: SegmentRef) {
		let track = e.borrow_el().track();
		if (track as usize) < e.borrow_el().score().staff_count() * constants::VOICES {
			// TODO: check if we replace something nongenerated. If yes, warn or smt
			assert!(e.borrow_el().elist.len() > track as usize, "Track lies outside configured amount of staves.");
			e.borrow_mut_el().elist[track as usize] = Some(c);
		}
	}

	pub fn remove(&mut self, e: &ElementRef) {
		match e {
			_ => {}
		}
	}

	pub fn insert_staff(&mut self, _staff: StaffId) {
		// TODO: impl
		unimplemented!()
	}

	pub fn remove_staff(&mut self, _staff: StaffId) {
		// TODO: impl
		unimplemented!()
	}
}

impl Element for Segment {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Segment }

	fn time(&self) -> Fraction {
		self.rel_time() + self.measure().with_d(|e| e.time(), Fraction::zero())
	}
}

impl SegmentTrait for Segment {
	fn segment(&self) -> Option<El<Segment>> { self.get_ref_ty() }
	fn measure(&self) -> Option<El<Measure>> { self.parent()?.try_into().ok() }
}

pub trait SegmentTrait: Element {
	fn segment(&self) -> Option<El<Segment>> {
		self.parent().and_then(|e| e.try_into().ok())
	}
	fn measure(&self) -> Option<El<Measure>> { self.segment()?.borrow_el().measure() }
	fn system(&self) -> Option<El<System>> { self.measure()?.borrow_el().system() }
}

bitflags! { pub struct SegmentTypeMask: u16 {
	const INVALID                   = 0x00000000;
	const BEGIN_BARLINE             = 0x00000001;
	const HEADER_CLEF               = 0x00000002;
	const KEYSIG                    = 0x00000004;
	const AMBITUS                   = 0x00000008;
	const TIMESIG                   = 0x00000010;
	const START_REPEAT_BARLINE      = 0x00000020;
	const CLEF                      = 0x00000040;
	const BARLINE                   = 0x00000080;
	const BREATH                    = 0x00000100;
	const CHORDREST                 = 0x00000200;
	const END_BARLINE               = 0x00000400;
	const KEYSIG_ANNOUNCE           = 0x00000800;
	const TIMESIG_ANNOUNCE          = 0x00001000;
	const BARLINE_TYPE              = Self::BEGIN_BARLINE.bits | Self::START_REPEAT_BARLINE.bits | Self::BARLINE.bits | Self::END_BARLINE.bits;
}}

impl From<SegmentType> for SegmentTypeMask {
	fn from(t: SegmentType) -> Self {
		match t {
			SegmentType::Invalid => SegmentTypeMask::INVALID,
			SegmentType::Barline => SegmentTypeMask::BARLINE,
			SegmentType::Chord => SegmentTypeMask::CHORDREST,
			SegmentType::Clef => SegmentTypeMask::CLEF,
			SegmentType::KeySig => SegmentTypeMask::KEYSIG,
			SegmentType::Rest => SegmentTypeMask::CHORDREST,
			SegmentType::TimeSig => SegmentTypeMask::TIMESIG,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::testing;
	use crate::score::*;

	#[test]
	fn test_add_chord() {
		let score = testing::setup_score();
		let chord = Chord::new(score.clone()).with_mut_i(|mut chord| {
			chord.set_visible(true);
		});
		let segment = Segment::new(score.clone());
		Segment::add(segment.clone(), chord.into());

		segment.with(|segment| {
			assert_eq!(segment.visible(), true)
		});
	}

	#[test]
	fn test_add_timeelem() {
		let _score = testing::setup_score();
		// TODO: test add time / key sig
	}
}