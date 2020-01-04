use crate::score::*;

#[derive(Debug, Clone)]
pub struct SymbolGroup {
	element: ElementData,

	leafs: Vec<ElementRef>,
	align: Align,
}

impl Element for SymbolGroup {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::SymbolGroup }
}