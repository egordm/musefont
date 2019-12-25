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
	fn layout(&mut self, data: &LayoutData) {
		unimplemented!()
	}

	fn draw(&self, painter: PainterRef) {
		painter.draw(DrawData::new(self.sym, *self.scale(), self.pos()))
	}
}
