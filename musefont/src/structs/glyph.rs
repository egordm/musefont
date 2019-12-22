use font_kit::canvas::Canvas;
use std::collections::HashMap;
use crate::types::*;
use crate::sym_id::SymId;

pub type GlyphId = SymId;

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphKey {
	id: GlyphId,
	size: Size2F,
	scale: f32,
}

#[derive(Debug)]
pub struct GlyphPixmap {
	canvas: Canvas,
	offset: Point2F,
}

pub type GlyphCache = HashMap<GlyphKey, GlyphPixmap>;