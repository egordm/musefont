pub use crate::*;
pub use super::*;
use crate::num_traits::FromPrimitive;
use font_kit::loaders::default::Font;
use std::{path::Path, fs::File};
use crate::constants::SPATIUM20;

macro_rules! impl_display {
    ($enum:ident, {$($variant:pat => $fmt_string:expr),+$(,)* }) => {
        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use self::$enum::*;
                match &self {
                    $(
                        $variant => write!(f, "{}", $fmt_string),
                    )+
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum FontLoadingError {
	IO(std::io::Error),
	Font(font_kit::error::FontLoadingError),
	Glyph(font_kit::error::GlyphLoadingError),
	Json(json::Error),
	Other(String),
}

impl std::error::Error for FontLoadingError {}

impl_display! { FontLoadingError, {
        IO(e) => format!("IO error: {}", e),
        Font(e) => format!("Font loading error: {}", e),
        Glyph(e) => format!("Glyph loading error: {}", e),
        Json(e) => format!("Json loading error: {}", e),
        Other(e) => format!("Error occurred: {}", e),
    }
}

type Error = FontLoadingError;

pub fn load(path: &Path, filename: &str, config: &FontMapping) -> Result<ScoreFont, Error> {
	let mut font_file = File::open(path.join(filename)).map_err(Error::IO)?;
	let font = Font::from_file(&mut font_file, 0).map_err(Error::Font)?;
	let mut font = ScoreFont::new(font);
	font.name = font.font.full_name();
	font.family = font.font.family_name();

	// Compute metrics for all symbols
	for (id, code) in config.sym_codes.iter().cloned().enumerate() {
		// TODO: can probably just join the array of symnames.
		let sym = SymName::from_usize(id).expect("SymId must correspond to a symbol");
		compute_metrics(sym, &mut font.symbols[id], code, &font.font)?;
	}

	// Load symbol data
	let meta_str = std::fs::read_to_string(path.join("metadata.json")).map_err(Error::IO)?;
	let meta = json::parse(&meta_str).map_err(Error::Json)?;
	for (sym_name, v) in meta["glyphsWithAnchors"].entries() {
		let sym_id = config.get_symid(sym_name);
		if SymName::NoSym == sym_id { continue; }
		parse_sym(&mut font.symbols[sym_id as usize], v)?;
	}

	// TODO: engravings

	// Load compound data and recalculte the bounding boxes
	for (sym_id, children) in COMPOSED_SYMBOLS.iter().cloned() {
		if !font.symbols[sym_id as usize].is_valid() {
			let bb = font.bounding_box_combined(children.iter().cloned().map(SymName::id), &Size2F::new(1., 1.));
			let sym = &mut font.symbols[sym_id as usize];
			sym.compound_ids = children.iter().cloned().map(SymName::id).collect();
			sym.bbox = bb;
		}
	}

	// TODO: style

	compute_metrics(SymName::Space, &mut font.symbols[SymName::Space as usize], 32, &font.font)?;
	Ok(font)
}


fn compute_metrics(_sym_name: SymName, sym: &mut Sym, code: u32, font: &Font) -> Result<(), Error> {
	if let Some(char) = std::char::from_u32(code) {
		if let Some(glyph_id) = font.glyph_for_char(char) {
			let down_scale = SPATIUM20 / font.metrics().units_per_em as f32;
			// typographic_bounds returns size of 1em defined by font->units_per_em
			let mut bb = font.typographic_bounds(glyph_id).map_err(Error::Glyph)? * down_scale;
			// Looks like one needs to flip the bb vertically?
			bb.origin.y = -(bb.size.height + bb.origin.y);
			sym.code = code as i32;
			sym.index = glyph_id;
			sym.bbox = bb;
			sym.advance = font.advance(glyph_id).map_err(Error::Glyph)?.x / down_scale;

			if _sym_name == SymName::Flag64thUp {
				let i = 0;
			}
		}
	}
	Ok(())
}

fn parse_sym(sym: &mut Sym, data: &json::JsonValue) -> Result<(), Error> {
	const SCALE: f32 = constants::SPATIUM20;
	const CSCALE: f32 = constants::SPATIUM20 / 1000.;
	for (k, v) in data.entries() {
		match k {
			"stemDownNW" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.stem_down_nw = Point2F::new(constants::DPI_F * x, constants::DPI_F * -y);
			},
			"stemUpSE" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.stem_up_se = Point2F::new(constants::DPI_F * x, constants::DPI_F * -y);
			},
			"cutOutNE" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.cut_out_ne = Point2F::new(SCALE * x, SCALE * -y);
			},
			"cutOutNW" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.cut_out_nw = Point2F::new(SCALE * x, SCALE * -y);
			},
			"cutOutSE" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.cut_out_se = Point2F::new(SCALE * x, SCALE * -y);
			},
			"cutOutSW" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.cut_out_sw = Point2F::new(SCALE * x, SCALE * -y);
			},
			_ => {},
		}
	}

	Ok(())
}