use crate::score::*;

/// One measure in a system
#[derive(Debug, Clone)]
pub struct Measure {
	element: ElementData,

	measure_data: MeasureData,
	segments: OrderedCollecton<SegmentRef>,

	mm_rest: Option<MeasureRefWeak>,

	timesig: Fraction,
	repeat_count: i32,

	no_mode: MeasureNumberMode,
}

impl Element for Measure {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Measure }
}

impl MeasureTrait for Measure {
	fn measure_data(&self) -> &MeasureData { &self.measure_data }
	fn measure_data_mut(&mut self) -> &mut MeasureData {&mut self.measure_data }
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