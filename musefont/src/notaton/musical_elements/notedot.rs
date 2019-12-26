use crate::*;

#[derive(Clone, Debug)]
pub struct NoteDot {
	element: Element
}

impl ElementTrait for NoteDot {
	fn el(&self) -> &Element { &self.element }
	fn el_mut(&mut self) -> &mut Element { &mut self.element }
	fn element_type(&self) -> ElementType { ElementType::NoteDot }
}