use crate::*;
use crate::score::*;

#[derive(Debug, Clone)]
pub struct Beam {
	element: ElementData,

	///  Must be sorted by tick
	chords: OrderedCollecton<ChordRef>,
	segments: Vec<LineF>,
	/// beam splits across systems
	fragments: Vec<BeamFragment>,
	direction: DirectionV,

	up: bool,
	/// equal spacing of elements
	distribute: bool,
	no_slope: bool,

	is_grace: bool,
	cross: bool,

	/// define "feather" beams
	grow_left: f32,
	/// define "feather" beams
	grow_right: f32,
	beam_dist: f32,

	min_move: i32,
	max_move: i32,
	max_duration: Duration,
	slope: f32,
}

impl Element for Beam {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Beam }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpannerSegmentType {
	SINGLE,
	BEGIN,
	MIDDLE,
	END
}

#[derive(Copy, Clone, Debug, PartialEq, Primitive, Eq)]
pub enum BeamMode {
	Auto = 0,
	Begin = 1,
	Mid = 2,
	End = 3,
	None = 4,
	Begin32 = 5,
	Begin64 = 6,
	Invalid = 7,
}

#[derive(Clone, Debug, Default)]
struct BeamFragment {
	py: [Point2F; 2],
}