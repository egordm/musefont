use crate::{ElementRef, Element, ElementGroup, SymId, ElementTrait, ElementType};

#[derive(Clone, Debug)]
pub struct Symbol {
	element: ElementGroup,
	sym: SymId,
}

impl ElementTrait for Symbol {
	fn el(&self) -> &Element { self.element.el() }

	fn el_mut(&mut self) -> &mut Element { self.element.el_mut() }

	fn element_type(&self) -> ElementType { ElementType::Symbol}
}