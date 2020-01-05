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
	tick: Fraction,
	ticks: Fraction,
	extra_leading_space: Spatium,
	stretch: bool,

	/// The list of annotations (read only)
	annotations: Vec<ElementRef>,
	/// Element storage, size = staves * VOICES
	elist: [Vec<ElementRef>; constants::VOICES],
	// size = staves
	//shapes: Vec<Shape>,
	/// size = staves
	dot_pos_x: Vec<f32>,
}

impl Segment {
	pub fn segment_type(&self) -> SegmentTypeMask { self.segment_type }
	pub fn set_segment_type(&mut self, v: SegmentTypeMask) { self.segment_type = v }

	pub fn time(&self) -> Fraction { self.tick }
	pub fn duration(&self) -> Fraction { self.ticks }
	pub fn set_duration(&mut self, v: Fraction) { self.ticks = v }

	pub fn extra_leading_space(&self) -> Spatium { self.extra_leading_space }
	pub fn set_extra_leading_space(&mut self, v: Spatium) { self.extra_leading_space = v }
	pub fn stretch(&self) -> bool { self.stretch }
	pub fn set_stretch(&mut self, v: bool) { self.stretch = v }

	pub fn annotations(&self) -> &Vec<ElementRef> { &self.annotations }
	pub fn set_annotations(&mut self, v: Vec<ElementRef>) { self.annotations = v }
	pub fn elements(&self) -> &[Vec<ElementRef>; constants::VOICES] { &self.elist }

	pub fn dot_pos_x(&self) -> &Vec<f32> { &self.dot_pos_x }
	pub fn set_dot_pos_x(&mut self, v: Vec<f32>) { self.dot_pos_x = v }

	pub fn next(&self) -> Option<El<Segment>> {
		self.measure()?.borrow_el().segment_next_iter(self.time()).skip(1).next().cloned()
	}
	pub fn prev(&self) -> Option<El<Segment>> {
		self.measure()?.borrow_el().segment_prev_iter(self.time()).skip(1).next().cloned()
	}
	pub fn next_type(&self, t: impl Into<SegmentTypeMask>) -> Option<El<Segment>> {
		let t = t.into();
		self.measure()?.borrow_el().segment_next_iter(self.time()).skip(1)
			.filter(|e| e.borrow_el().segment_type() & t == t).next().cloned()
	}
	pub fn prev_type(&self, t: impl Into<SegmentTypeMask>) -> Option<El<Segment>> {
		let t = t.into();
		self.measure()?.borrow_el().segment_prev_iter(self.time()).skip(1)
			.filter(|e| e.borrow_el().segment_type() & t == t).next().cloned()
	}

	pub fn next_chordrest(&self, track: i32, backwards: bool) -> Option<ChordRef> {
		let f = |segment: &El<Segment>| {
			for e in &segment.borrow_el().elements()[track as usize] {
				if e.get_type() != ElementType::Chord && e.get_type() != ElementType::Rest { continue; }
				if let Ok(res) = ChordRef::try_from(e.clone()) { return Some(res); }
			}
			return None;
		};

		if backwards {
			for segment in self.measure()?.borrow_el().segment_prev_iter(self.time()) {
				if let Some(res) = f(segment) { return Some(res)}
			}
		} else {
			for segment in self.measure()?.borrow_el().segment_next_iter(self.time()) {
				if let Some(res) = f(segment) { return Some(res)}
			}
		}
		return None;
	}
}

impl Element for Segment {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Segment }
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

bitflags! { pub struct SegmentTypeMask: i32 {
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
	const ALL                       = -1;
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
			SegmentType::Segment => SegmentTypeMask::INVALID,
			SegmentType::TimeSig => SegmentTypeMask::TIMESIG,
		}
	}
}