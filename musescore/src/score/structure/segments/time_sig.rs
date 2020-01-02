use crate::score::*;

#[derive(Debug, Clone)]
pub struct TimeSig {
	node: SegmentNodeData,
}

impl SegmentTrait for TimeSig {

}

impl SegmentNode for TimeSig {
	fn data(&self) -> &SegmentNodeData { &self.node }
	fn data_mut(&mut self) -> &mut SegmentNodeData { &mut self.node }
}