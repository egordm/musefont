use crate::*;
use super::*;
use font_kit::{loaders::freetype::Font, canvas::Canvas, hinting::HintingOptions, loader::FontTransform};

#[derive(Clone)]
pub struct ScoreFont {
	pub(super) name: String,
	pub(super) family: String,
	pub(super) symbols: Vec<Sym>,
	pub(super) cache: GlyphCache,
	pub(super) font: Font,
}

unsafe impl Send for ScoreFont {}

impl ScoreFont {
	pub fn new(font: Font) -> Self {
		let mut symbols = Vec::with_capacity(SYMBOL_COUNT);
		symbols.resize_with(SYMBOL_COUNT, || Sym::default());

		Self {
			name: String::default(),
			family: String::default(),
			symbols,
			cache: GlyphCache::new(),
			font
		}
	}

	pub fn name(&self) -> &String { &self.name }
	pub fn family(&self) -> &String { &self.family }
	pub fn sym(&self, id: impl Into<SymId>) -> &Sym { &self.symbols[id.into() as usize] }

	pub fn bounding_box(&self, id: impl Into<SymId>, mag: &Size2F) -> RectF {
		let bb = self.sym(id).bbox();
		RectF::new(bb.origin, Size2F::new(bb.size.width * mag.width, bb.size.height * mag.height))
	}
	pub fn bounding_box_combined(&self, it: impl IntoIterator<Item=impl Into<SymId> + Copy>, mag: &Size2F) -> RectF {
		let (mut pos, mut bb) = (VEC_ZERO, RectF::default());
		for sym_id in it {
			bb = bb.union(&self.bounding_box(sym_id, mag).translate(pos));
			pos.x += self.advance(sym_id, mag.width);
		}
		bb
	}

	pub fn advance(&self, id: impl Into<SymId>, mag: f32) -> f32 {
		self.sym(id).advance() * mag
	}
	pub fn width(&self, id: impl Into<SymId>, mag: f32) -> f32 {
		self.bounding_box(id, &Size2F::new(mag, mag)).size.width
	}
	pub fn height(&self, id: impl Into<SymId>, mag: f32) -> f32 {
		self.bounding_box(id, &Size2F::new(mag, mag)).size.height
	}
	pub fn stem_down_nw(&self, id: impl Into<SymId>, mag: f32) -> Point2F { self.sym(id).stem_down_nw() * mag }
	pub fn stem_up_se(&self, id: impl Into<SymId>, mag: f32) -> Point2F {
		self.sym(id).stem_up_se() * mag
	}
	pub fn cut_out_ne(&self, id: impl Into<SymId>, mag: f32) -> Point2F {
		*self.sym(id).cut_out_ne() * mag
	}
	pub fn cut_out_nw(&self, id: impl Into<SymId>, mag: f32) -> Point2F {
		*self.sym(id).cut_out_nw() * mag
	}
	pub fn cut_out_se(&self, id: impl Into<SymId>, mag: f32) -> Point2F {
		*self.sym(id).cut_out_se() * mag
	}
	pub fn cut_out_sw(&self, id: impl Into<SymId>, mag: f32) -> Point2F {
		*self.sym(id).cut_out_se() * mag
	}

	pub fn pixmap(&mut self, id: impl Into<SymId> + Copy, scale: &Size2F, rasterization_options: RasterizationOptions, format: Format) -> Option<&GlyphPixmap> {
		let key = GlyphKey::new(id.into(), *scale);
		if !self.cache.contains_key(&key) && self.sym(id).is_valid() {
			let glyph_id = self.sym(id).index;
			let transform = FontTransform::new(scale.width, 0., 0., scale.height);
			let hinting_options = HintingOptions::None;
			let point_size = self.font.metrics().units_per_em as f32 / 10.;

			let bounds = self.font.raster_bounds(
				glyph_id, point_size, &transform, &POINT_ZERO,
				hinting_options, rasterization_options,
			).ok()?;

			let size = Size2U::new(bounds.size.width as u32, bounds.size.height as u32);
			let origin = Point2F::new(bounds.origin.x as f32, -bounds.origin.y as f32);
			let mut canvas = Canvas::new(&size, format);

			self.font.rasterize_glyph(
				&mut canvas, glyph_id, point_size, &transform, &origin,
				hinting_options, rasterization_options,
			).ok()?;

			let glyph = GlyphPixmap::new(canvas, Point2F::new(origin.x, -origin.y));
			self.cache.insert(key.clone(), glyph);
		}

		self.cache.get(&key)
	}

	// TODO: fallback font
}