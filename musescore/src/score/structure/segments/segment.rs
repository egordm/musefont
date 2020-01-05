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

	segment_type: SegmentType,
	/// midi tick position (read only)
	tick: Fraction,
	ticks: Fraction,
	extra_leading_space: Spatium,
	stretch: bool,

	/// The list of annotations (read only)
	annotations: Vec<ElementRef>,
	/// Element storage, size = staves * VOICES
	elist: Vec<ElementRef>,
	// size = staves
	//shapes: Vec<Shape>,
	/// size = staves
	dot_pos_x: Vec<f32>,
}

impl Element for Segment {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Segment }
}

impl SegmentTrait for Segment {
	fn segment(&self) -> Option<El<Segment>> { self.get_ref_ty() }
	fn measure(&self) -> Option<MeasureRef> {
		self.parent().and_then(|e| MeasureRef::try_from(e).ok())
	}
}

pub trait SegmentTrait: Element {
	fn segment(&self) -> Option<El<Segment>> {
		self.parent().and_then(|e| e.try_into().ok())
	}
	fn measure(&self) -> Option<MeasureRef> {
		MeasureRef::try_from(self.segment()?.borrow_el().parent()?).ok()
	}
	fn system(&self) -> Option<El<System>> {
		self.measure()?.as_trait().parent()?.try_into().ok()
	}
}