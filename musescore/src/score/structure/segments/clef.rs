use crate::score::*;
use crate::font::SymName;

#[derive(Debug, Clone)]
pub struct Clef {
	element: ElementData,
	segment_data: SegmentData,

	sym_id: SymName,
	show_courtesy: bool,
	small: bool,

	clef_types: ClefTypeList,
}

impl Element for Clef {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}

impl SegmentTrait for Clef {
	fn segment_data(&self) -> &SegmentData { &self.segment_data }
	fn segment_data_mut(&mut self) -> &mut SegmentData { &mut self.segment_data }
}