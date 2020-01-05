use crate::*;

#[derive(Clone, Debug)]
pub struct Part {
	element: Element,

	part_name: String,
	instruments: InstrumentList,
	staves: Vec<Elem<Staff>>,

	show: bool,
}

impl ElementTrait for Part {
	fn el(&self) -> &Element { &self.element }
	fn el_mut(&mut self) -> &mut Element { &mut self.element }
	fn element_type(&self) -> ElementType { ElementType::Staff }
}
