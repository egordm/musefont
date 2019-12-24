use crate::*;

#[derive(Clone, Debug)]
pub struct SymbolGroup {
	element: Element,
	leafs: Vec<Symbol>,
}

impl SymbolGroup {
	pub fn leaves(&self) -> &Vec<Symbol> { &self.leafs }

	/*pub fn add(&mut self, el: Symbol) -> bool {
		if el.element_type() == ElementType::Symbol {
			el.set_parent(Some())

			true
		} else {
			false
		}
	}*/
}

impl ElementTrait for SymbolGroup {
	fn el(&self) -> &Element { &self.element }

	fn el_mut(&mut self) -> &mut Element { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Group }
}