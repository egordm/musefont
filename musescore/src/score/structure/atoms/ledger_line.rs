use crate::score::*;

/// # [LedgerLine](https://en.wikipedia.org/wiki/Ledger_line)
/// Graphic representation of a ledger line.
#[derive(Debug, Clone)]
pub struct LedgerLine {
	element: ElementData,

	width: f32,
	len: f32,
	next: Option<El<LedgerLine>>,
	vertical: bool,
}

impl Element for LedgerLine {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}