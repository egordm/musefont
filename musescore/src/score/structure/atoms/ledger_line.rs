use crate::score::*;

/// # [LedgerLine](https://en.wikipedia.org/wiki/Ledger_line)
/// Graphic representation of a ledger line.
#[derive(Debug, Clone)]
pub struct LedgerLine {
	width: f32,
	len: f32,
	next: Option<El<LedgerLine>>,
	vertical: bool,
}