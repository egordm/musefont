use crate::*;
use font_kit::loaders::freetype::Font;

pub struct ScoreFont {
	pub(crate) name: String,
	pub(crate) family: String,
	pub(crate) symbols: Vec<Sym>,
	pub(crate) cache: GlyphCache,
	pub(crate) font: Font,
}

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

	pub fn name(&self) -> &str { &self.name }

	pub fn family(&self) -> &str { &self.family }

	pub fn sym(&self, id: SymId) -> &Sym {
		&self.symbols[id as usize]
	}

	pub fn bounding_box(&self, id: SymId, mag: &Size2F) -> RectF {
		let bb = self.sym(id).bbox();
		RectF::new(bb.origin, Size2F::new(bb.size.width * mag.width, bb.size.height * mag.height))
	}

	pub fn advance(&self, id: SymId, mag: f32) -> f32 {
		self.sym(id).advance() * mag
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
}