use crate::score::*;

#[derive(Debug, Clone)]
pub struct Rest {
	node: SegmentNodeData,
}

impl SegmentTrait for Rest {

}

impl SegmentNode for Rest {
	fn data(&self) -> &SegmentNodeData { &self.node }
	fn data_mut(&mut self) -> &mut SegmentNodeData { &mut self.node }
}