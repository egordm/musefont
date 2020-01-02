use crate::score::*;

#[derive(Debug, Clone)]
pub struct Chord {
	node: SegmentNodeData,
}

impl SegmentTrait for Chord {

}

impl SegmentNode for Chord {
	fn data(&self) -> &SegmentNodeData { &self.node }
	fn data_mut(&mut self) -> &mut SegmentNodeData { &mut self.node }
}