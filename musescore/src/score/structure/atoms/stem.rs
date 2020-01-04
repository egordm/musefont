use crate::*;
use crate::score::*;

#[derive(Debug, Clone)]
pub struct Stem {
	element: ElementData,

	line: LineF,
	line_width: f32,
	user_len: f32,
	len: f32,
}

impl Element for Stem {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Stem }
}