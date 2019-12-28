use crate::*;


#[derive(Clone, Debug)]
pub enum DrawIns {
	Symbol(SymId, Size2F, Point2F),
	Line(LineF, f32),
}

pub trait Painter {
	fn draw(&mut self, data: DrawIns);

	fn translate(&mut self, pt: Vec2F);
}

pub type PainterRef<'a> = &'a mut dyn Painter;

pub trait Drawable: ElementTrait {
	fn layout(&mut self);

	fn draw(&self, painter: PainterRef);
}