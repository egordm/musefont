use crate::drawing::Path;
use crate::font::SymName;
use crate::{Size2F, Point2F, RectF};

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
	Path(Path),
	Rect(RectF, f32),
	Symbol(Symbol),
	Point(Point2F, f32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
	symbol: SymName,
	sym_char: char,
	scale: Size2F,
	pos: Point2F,
}

impl Into<Instruction> for Symbol {
	fn into(self) -> Instruction { Instruction::Symbol(self) }
}

impl Symbol {
	pub fn new(symbol: SymName, sym_char: char, scale: Size2F, pos: Point2F) -> Self {
		Self {
			symbol,
			sym_char,
			scale,
			pos
		}
	}

	pub fn sym(&self) -> &SymName { &self.symbol }
	pub fn sym_char(&self) -> char { self.sym_char }
	pub fn scale(&self) -> &Size2F { &self.scale }
	pub fn pos(&self) -> &Point2F { &self.pos }
}