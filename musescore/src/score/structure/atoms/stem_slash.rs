use crate::*;
use crate::score::*;

#[derive(Debug, Clone)]
pub struct StemSlash {
	element: ElementData,

	line: LineF,
}

impl StemSlash {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		line: Default::default()
	})}

	pub fn set_line(&mut self, v: LineF) { self.line = v }
}

impl Element for StemSlash {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::StemSlash }
}

impl AtomTrait for StemSlash {

}
