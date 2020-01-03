use crate::*;
use crate::score::*;

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
pub struct SegmentData {
	segment_type: SegmentType,
	/// midi tick position (read only)
	tick: Fraction,
	ticks: Fraction,
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

pub trait SegmentTrait {
	fn segment_data(&self) -> &SegmentData;
	fn segment_data_mut(&mut self) -> &mut SegmentData;
}