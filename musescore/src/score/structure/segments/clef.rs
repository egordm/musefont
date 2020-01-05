use crate::score::*;
use crate::font::SymName;

#[derive(Debug, Clone)]
pub struct Clef {
	element: ElementData,

	sym_id: SymName,
	show_courtesy: bool,
	small: bool,

	clef_types: ClefTypeList,
}

impl Element for Clef {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Clef }
}

impl SegmentTrait for Clef {
}