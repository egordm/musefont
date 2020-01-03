use crate::score::*;

#[derive(Debug, Clone)]
pub struct SymbolGroup {
	leafs: Vec<ElementRef>,
	align: Align,
}