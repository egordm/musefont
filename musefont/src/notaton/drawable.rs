use crate::*;


#[derive(Clone, Debug)]
pub struct DrawData {
	symid: SymId,
	scale: Size2F,
	pos: Point2F,
}

impl DrawData {
	pub fn new(symid: SymId, scale: Size2F, pos: Point2F) -> Self { Self { symid, scale, pos }}
}

pub struct LayoutData<'a> {
	font: &'a ScoreFont
}

impl<'a> LayoutData<'a> {
	pub fn font(&self) -> &ScoreFont { self.font }
}

pub trait Painter {
	fn draw(&mut self, data: DrawData);

	fn translate(&mut self, pt: Vec2F);
}

pub type PainterRef<'a> = &'a mut dyn Painter;

pub trait Drawable: ElementTrait {
	fn layout(&mut self, data: &LayoutData);

	fn draw(&self, painter: PainterRef);
}