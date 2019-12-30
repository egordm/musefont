use crate::*;

#[derive(Clone, Debug)]
pub struct SymbolGroup {
	element: Element,
	leafs: Vec<Symbol>,
}

impl SymbolGroup {
	pub(crate) fn default(score: Score) -> Self { Self {
		element: Element::new(score),
		leafs: Vec::new(),
	}}

	pub fn new(score: Score) -> Elem<Self> { Elem::new(Self::default(score))}
}

impl SymbolGroup {
	pub fn leaves(&self) -> &Vec<Symbol> { &self.leafs }
}

impl ElementTrait for SymbolGroup {
	fn el(&self) -> &Element { &self.element }
	fn el_mut(&mut self) -> &mut Element { &mut self.element }
	fn element_type(&self) -> ElementType { ElementType::Group }
}