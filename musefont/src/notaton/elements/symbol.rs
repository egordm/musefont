use crate::*;

#[derive(Clone, Debug)]
pub struct Symbol {
	element: SymbolGroup,
	sym: SymId,
}

impl ElementTrait for Symbol {
	fn el(&self) -> &Element { self.element.el() }
	fn el_mut(&mut self) -> &mut Element { self.element.el_mut() }
	fn element_type(&self) -> ElementType { ElementType::Symbol}
}

impl Drawable for Symbol {
	fn layout(&mut self) {
		unimplemented!()
	}

	fn draw(&self, painter: PainterRef) {
		painter.draw(DrawIns::Symbol(self.sym, Size2F::new(self.scale(), self.scale()), self.pos()))
	}
}
