use crate::score::*;

/// One measure in a system
#[derive(Debug, Clone)]
pub struct Measure {
	measure_data: MeasureData,
	segments: OrderedCollecton<SegmentRef>,

	mm_rest: Option<MeasureRefWeak>,

	timesig: Fraction,
	repeat_count: i32,

	no_mode: MeasureNumberMode,
}

impl MeasureTrait for Measure {

}

#[derive(Debug, Clone, Copy)]
pub enum MeasureNumberMode {
	/// show measure number depending on style
	Auto,
	/// always show measure number
	Show,
	/// don’t show measure number
	Hide
}