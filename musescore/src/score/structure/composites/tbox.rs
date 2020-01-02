use crate::score::*;

#[derive(Debug, Clone)]
pub struct VBox {
	node: MeasureNodeData,
}

impl MeasureTrait for VBox {

}

impl MeasureNode for VBox {
	fn data(&self) -> &MeasureNodeData { &self.node }
	fn data_mut(&mut self) -> &mut MeasureNodeData { &mut self.node }
}

#[derive(Debug, Clone)]
pub struct HBox {
	node: MeasureNodeData,
}

impl MeasureTrait for HBox {

}

impl MeasureNode for HBox {
	fn data(&self) -> &MeasureNodeData { &self.node }
	fn data_mut(&mut self) -> &mut MeasureNodeData { &mut self.node }
}