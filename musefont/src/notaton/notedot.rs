use crate::*;

#[derive(Clone)]
pub struct NoteDot {
	element: Element
}

impl ElementTrait for NoteDot {
	fn el(&self) -> &Element { &self.element }

	fn el_mut(&mut self) -> &mut Element { &mut self.element }
}