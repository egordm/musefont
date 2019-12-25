use font_kit::{loaders::freetype::Font, canvas::Canvas, hinting::HintingOptions, loader::FontTransform};
use crate::*;

#[derive(Clone)]
pub struct ScoreFont {
	pub(crate) name: String,
	pub(crate) family: String,
	pub(crate) symbols: Vec<Sym>,
	pub(crate) cache: GlyphCache,
	pub(crate) font: Font,
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

	pub fn sym(&self, id: SymId) -> &Sym {
		&self.symbols[id as usize]
	}

	pub fn bounding_box(&self, id: SymId, mag: &Size2F) -> RectF {
		let bb = self.sym(id).bbox();
		RectF::new(bb.origin, Size2F::new(bb.size.width * mag.width, bb.size.height * mag.height))
	}

	pub fn bounding_box_combined(&self, it: impl IntoIterator<Item=SymId>, mag: &Size2F) -> RectF {
		let mut pos = Point2F::default().to_vector();
		let mut ret = RectF::default();
		for sym_id in it {
			ret = ret.union(&self.bounding_box(sym_id, mag).translate(pos));
			pos.x += self.advance(sym_id, mag.width);
		}
		ret
	}

	pub fn advance(&self, id: SymId, mag: f32) -> f32 {
		self.sym(id).advance() * mag
	}

	pub fn width(&self, id: SymId, mag: f32) -> f32 {
		self.bounding_box(id, &Size2F::new(mag, mag)).size.width
	}

	// TODO: fallback font

	pub fn stem_down_nw(&self, id: SymId, mag: f32) -> Point2F {
		*self.sym(id).stem_down_nw() * mag
	}

	pub fn stem_up_se(&self, id: SymId, mag: f32) -> Point2F {
		*self.sym(id).stem_up_se() * mag
	}

	pub fn cut_out_ne(&self, id: SymId, mag: f32) -> Point2F {
		*self.sym(id).cut_out_ne() * mag
	}

	pub fn cut_out_nw(&self, id: SymId, mag: f32) -> Point2F {
		*self.sym(id).cut_out_nw() * mag
	}

	pub fn cut_out_se(&self, id: SymId, mag: f32) -> Point2F {
		*self.sym(id).cut_out_se() * mag
	}

	pub fn cut_out_sw(&self, id: SymId, mag: f32) -> Point2F {
		*self.sym(id).cut_out_se() * mag
	}

	pub fn pixmap(&mut self, id: SymId, scale: &Size2F, point_size: f32, rasterization_options: RasterizationOptions, format: Format) -> Option<&GlyphPixmap> {
		let key = GlyphKey::new(id, *scale, point_size);
		if !self.cache.contains_key(&key) && self.sym(id).is_valid() {
			let test = self.sym(id);
			let glyph_id = self.sym(id).index;
			let transform = FontTransform::new(scale.width, 0., 0., scale.height);
			let hinting_options = HintingOptions::None;

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
}