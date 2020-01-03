use crate::score::*;
use crate::font::*;

#[derive(Debug, Clone)]
pub struct Symbol {
	element: SymbolGroup,
	sym: SymId,
}