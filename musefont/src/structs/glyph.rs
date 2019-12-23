use font_kit::canvas::Canvas;
use std::collections::HashMap;
use crate::types::*;
use crate::sym_id::SymId;
use std::hash::{Hash, Hasher};

pub type GlyphId = SymId;

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphKey {
	id: GlyphId,
	size: Size2F,
	scale: f32,
}

impl Eq for GlyphKey {}

impl Hash for GlyphKey {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let h = ((self.id as u64) << 16) + (((self.size.width * 100.) as u64) << 8) + (self.size.height as u64);
		state.write_u64(h);
	}
}

#[derive(Debug)]
pub struct GlyphPixmap {
	canvas: Canvas,
	offset: Point2F,
}

pub type GlyphCache = HashMap<GlyphKey, GlyphPixmap>;