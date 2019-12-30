use crate::*;

#[derive(Clone, Debug)]
pub struct Symbol {
	element: SymbolGroup,
	sym: SymId,
}

impl Symbol {
	pub(crate) fn default(score: Score) -> Self {
		Self {
			element: SymbolGroup::default(score),
			sym: SymIdent::NoSym as SymId
		}
	}

	pub fn new(score: Score) -> Elem<Self> { Elem::new(Self::default(score)) }

	pub fn sym(&self) -> SymId { self.sym }
	pub fn set_sym(&mut self, sym_id: SymId) { self.sym = sym_id }
}

impl ElementTrait for Symbol {
	fn el(&self) -> &Element { self.element.el() }
	fn el_mut(&mut self) -> &mut Element { self.element.el_mut() }
	fn element_type(&self) -> ElementType { ElementType::Symbol }
}

impl Drawable for Symbol {
	fn layout(&mut self) {
		let scale = self.scale();
		let bb = self.score().font().bounding_box(
			self.sym(),
			&Size2F::new(scale, scale));
		self.set_bbox(bb);
	}

	fn draw(&self, painter: PainterRef) {
		if SymIdent::NoSym != self.sym {
			painter.draw(DrawIns::Symbol(self.sym, Size2F::new(self.scale(), self.scale()), self.pos()))
		}
	}
}
