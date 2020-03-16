use crate::drawing::Instruction;
use crate::{Vec2F, Color};
use crate::font::ScoreFont;

pub trait Painter {
	fn draw(&mut self, i: Instruction);
	fn translate(&mut self, m: Vec2F);
	fn set_scale(&mut self, scale: f32);
	fn set_dpi(&mut self, dpi: f32);
	fn set_color(&mut self, c: Color);
	fn set_score_font(&mut self, f: ScoreFont);
}

pub type PainterRef<'a> = &'a mut dyn Painter;