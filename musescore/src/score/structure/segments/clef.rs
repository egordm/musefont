use crate::score::*;

#[derive(Debug, Clone)]
pub struct Clef {
	node: SegmentNodeData,
}

impl SegmentTrait for Clef {

}

impl SegmentNode for Clef {
	fn data(&self) -> &SegmentNodeData { &self.node }
	fn data_mut(&mut self) -> &mut SegmentNodeData { &mut self.node }
}