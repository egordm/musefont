use crate::*;
use crate::score::*;

#[derive(Debug, Clone)]
pub struct StemSlash {
	element: ElementData,

	line: LineF,
}

impl Element for StemSlash {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::StemSlash }
}