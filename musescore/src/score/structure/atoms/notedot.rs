use crate::score::*;

#[derive(Debug, Clone)]
pub struct NoteDot {
	element: ElementData,
}

impl NoteDot {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
	})}
}

impl Element for NoteDot {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::NoteDot }
}

impl AtomTrait for NoteDot {

}
