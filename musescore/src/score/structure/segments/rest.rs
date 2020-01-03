use crate::score::*;

#[derive(Debug, Clone)]
pub struct Rest {
	// TODO: Duration Element

	pub(super) elements: Vec<ElementRef>,
	pub(super) duration_type: Duration,
	/// -1, 0, +1, used for crossbeaming
	pub(super) staff_move: i32,

	// TODO: tab duraiton symbol && lyrics

	pub(super) beam: Option<ElWeak<Beam>>,
	pub(super) beam_mode: BeamMode,

	/// actual stem direction
	pub(super) up: bool,
	pub(super) small: bool,

	/// CrossMeasure: combine 2 tied notes if across a bar line and can be combined in a single duration
	/// 0: no cross-measure modification; 1: 1st note of a mod.; -1: 2nd note
	pub(super) cross_measure: CrossMeasure,
	/// the total Duration type of the combined notes
	pub(super) cross_measure_tdur: Duration,
}

impl SegmentTrait for Rest {

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CrossMeasure {
	Unknown = -1,
	None = 0,
	First = 1,
	Second = 2
}