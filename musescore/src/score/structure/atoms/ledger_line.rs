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

impl LedgerLine {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		width: 0.0,
		len: 0.0,
		next: None,
		vertical: false
	})}

	pub fn width(&self) -> f32 { self.width }
	pub fn set_width(&mut self, v: f32) { self.width = v }

	pub fn len(&self) -> f32 { self.len }
	pub fn set_len(&mut self, v: f32) { self.len = v }

	pub fn next(&self) -> &Option<El<LedgerLine>> { &self.next }
	pub fn set_next(&mut self, v: Option<El<LedgerLine>>) { self.next = v }

	pub fn vertical(&self) -> bool { self.vertical }
	pub fn set_vertical(&mut self, v: bool) { self.vertical = v }
}

impl Element for LedgerLine {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::LedgerLine }
}

impl AtomTrait for LedgerLine {

}