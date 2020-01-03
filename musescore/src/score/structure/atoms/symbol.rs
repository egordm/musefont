use crate::score::*;
use crate::font::*;

#[derive(Debug, Clone)]
pub struct Symbol {
	element: SymbolGroup,
	sym: SymId,
}

impl Element for Symbol {
	fn el_data(&self) -> &ElementData { self.element.el_data() }
	fn el_data_mut(&mut self) -> &mut ElementData { self.element.el_data_mut() }
}