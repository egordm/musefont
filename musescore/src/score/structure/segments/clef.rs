use crate::score::*;
use crate::font::SymName;

#[derive(Debug, Clone)]
pub struct Clef {
	sym_id: SymName,
	show_courtesy: bool,
	small: bool,

	clef_types: ClefTypeList,
}

impl SegmentTrait for Clef {

}