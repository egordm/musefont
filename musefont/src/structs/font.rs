use crate::{Sym, GlyphCache};
use font_kit::loaders::freetype::Font;

pub struct ScoreFont {
	name: String,
	family: String,
	symbols: Vec<Sym>,
	cache: GlyphCache,
	font: Font,
}

impl ScoreFont {
	pub fn name(&self) -> &str { &self.name }

	pub fn family(&self) -> &str { &self.family }
}