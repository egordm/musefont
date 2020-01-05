use crate::score::*;

#[derive(Debug, Clone)]
pub struct SymbolGroup {
	element: ElementData,

	leafs: Vec<ElementRef>,
	align: Align,
}

impl SymbolGroup {
	pub fn default(score: Score) -> Self { Self {
		element: ElementData::new(score),
		leafs: vec![],
		align: Align::LEFT | Align::BASELINE
	}}

	pub fn new(score: Score) -> El<Self> { new_element(Self::default(score))}

	pub fn align(&self) -> &Align { &self.align }
	pub fn set_align(&mut self, v: Align) { self.align = v }

	pub fn leafs(&self) -> &Vec<ElementRef> { &self.leafs }
	pub fn set_leafs(&mut self, v: Vec<ElementRef>) { self.leafs = v }
}

impl Element for SymbolGroup {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::SymbolGroup }
}

impl AtomTrait for SymbolGroup {

}
