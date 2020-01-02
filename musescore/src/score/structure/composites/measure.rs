use crate::score::*;

pub trait MeasureTrait: MeasureNode {

}

#[derive(Debug, Clone)]
pub struct Measure {
	node: MeasureNodeData,
	segments: SegmentList,
}

impl MeasureTrait for Measure {

}

impl MeasureNode for Measure {
	fn data(&self) -> &MeasureNodeData { &self.node }
	fn data_mut(&mut self) -> &mut MeasureNodeData { &mut self.node }
}