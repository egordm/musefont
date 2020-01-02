use crate::score::*;

#[derive(Debug, Clone)]
pub struct Barline {
	node: SegmentNodeData,
}

impl SegmentTrait for Barline {

}

impl SegmentNode for Barline {
	fn data(&self) -> &SegmentNodeData { &self.node }
	fn data_mut(&mut self) -> &mut SegmentNodeData { &mut self.node }
}