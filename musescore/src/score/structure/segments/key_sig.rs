use crate::score::*;

#[derive(Debug, Clone)]
pub struct KeySig {
	node: SegmentNodeData,
}

impl SegmentTrait for KeySig {

}

impl SegmentNode for KeySig {
	fn data(&self) -> &SegmentNodeData { &self.node }
	fn data_mut(&mut self) -> &mut SegmentNodeData { &mut self.node }
}