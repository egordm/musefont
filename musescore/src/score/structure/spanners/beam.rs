use crate::*;
use crate::score::*;

#[derive(Debug, Clone)]
pub struct Beam {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpannerSegmentType {
	SINGLE,
	BEGIN,
	MIDDLE,
	END
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BeamMode {
	Auto,
	Begin,
	Mid,
	End,
	None,
	Begin32,
	Begin64,
	Invalid
}

#[derive(Clone, Debug, Default)]
struct BeamFragment {
	py: [Point2F; 2],
}