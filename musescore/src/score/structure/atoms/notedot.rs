use crate::score::*;

#[derive(Debug, Clone)]
pub struct NoteDot {
	element: ElementData,
}

impl Element for NoteDot {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::NoteDot }
}