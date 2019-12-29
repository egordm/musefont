use font_kit::canvas::Canvas;
use std::collections::HashMap;
use crate::types::*;
use crate::sym_id::SymId;
use std::hash::{Hash, Hasher};

pub type GlyphId = SymId;

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphKey {
	id: GlyphId,
	scale: Size2F,
}

impl GlyphKey {
	pub fn new(id: GlyphId, scale: Size2F) -> GlyphKey {
		GlyphKey { id, scale }
	}
}

impl Eq for GlyphKey {}

impl Hash for GlyphKey {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let h = ((self.id as u64) << 16) + (((self.scale.width * 100.) as u64) << 8) + (self.scale.height as u64);
		state.write_u64(h);
	}
}

#[derive(Debug)]
pub struct GlyphPixmap {
	canvas: Canvas,
	offset: Point2F,
}

impl Clone for GlyphPixmap {
	fn clone(&self) -> Self {
		let mut canvas = Canvas::new(&self.canvas.size, self.canvas.format);
		canvas.stride = self.canvas.stride;
		canvas.pixels = self.canvas.pixels.clone();
		Self::new(canvas, self.offset)
	}
}

impl Default for GlyphPixmap {
	fn default() -> Self {
		Self::new(Canvas::new(&Size2U::default(), Format::A8), Point2F::default())
	}
}

impl GlyphPixmap {
	pub fn new(canvas: Canvas, offset: Point2F) -> Self {
		Self { canvas, offset }
	}

	pub fn canvas(&self) -> &Canvas { &self.canvas }

	pub fn offset(&self) -> &Point2F { &self.offset }

	pub fn stride(&self) -> usize { self.canvas.stride }

	pub fn size(&self) -> Size2U { self.canvas.size }

	pub fn width(&self) -> u32 { self.canvas.size.width }

	pub fn height(&self) -> u32 { self.canvas.size.height }

	pub fn format(&self) -> Format { self.canvas.format }

	pub fn pixels(&self) -> &Vec<u8> { &self.canvas.pixels }
}

pub type GlyphCache = HashMap<GlyphKey, GlyphPixmap>;