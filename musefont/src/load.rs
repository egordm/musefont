use font_kit::loaders::freetype::Font;
use std::{path::Path, fs::File};
use crate::*;

type Error = FontLoadingError;

pub fn load_font(path: &Path, filename: &str, config: &FontConfig) -> Result<ScoreFont, Error> {
	let mut font_file = File::open(path.join(filename)).map_err(Error::IO)?;
	let font = Font::from_file(&mut font_file, 0).map_err(Error::Font)?;
	let mut font = ScoreFont::new(font);
	font.name = font.font.full_name();
	font.family = font.font.family_name();

	// Compute metrics for all symbols
	for (id, code) in config.sym_codes.iter().cloned().enumerate() {
		compute_metrics(&mut font.symbols[id], code, &font.font)?;
	}

	let meta_str = std::fs::read_to_string(path.join("metadata.json")).map_err(Error::IO)?;
	let meta = json::parse(&meta_str).map_err(Error::Json)?;

	// Load symbol data
	for (sym_name, v) in meta["glyphsWithAnchors"].entries() {
		let sym_id = config.get_symid(sym_name);
		if SymIdent::NoSym == sym_id { continue; }
		parse_sym(&mut font.symbols[sym_id as usize], v)?;
	}

	// TODO: engravings

	// Load compound data and recalculte the bounding boxes
	for (sym_id, children) in COMPOSED_SYMBOLS.iter().cloned() {
		if !font.symbols[sym_id as usize].is_valid() {
			let bb = font.bounding_box_combined(children.iter().cloned().map(SymIdent::id), &Size2F::new(1., 1.));
			let sym = &mut font.symbols[sym_id as usize];
			sym.compound_ids = children.iter().cloned().map(SymIdent::id).collect();
			sym.bbox = bb;

		}
	}

	// TODO: style

	compute_metrics(&mut font.symbols[SymIdent::Space as usize], 32, &font.font)?;
	Ok(font)
}

fn compute_metrics(sym: &mut Sym, code: u32, font: &Font) -> Result<(), Error> {
	if let Some(char) = std::char::from_u32(code) {
		if let Some(glyph_id) = font.glyph_for_char(char) {
			let down_scale = 10.;
			// typographic_bounds returns size of 1em defined by font->units_per_em
			let bb = font.typographic_bounds(glyph_id).map_err(Error::Glyph)? / down_scale;
			sym.code = code as i32;
			sym.index = glyph_id;
			sym.bbox = bb;
			sym.advance = font.advance(glyph_id).map_err(Error::Glyph)?.x / down_scale;
		}
	}
	Ok(())
}

fn parse_sym(sym: &mut Sym, data: &json::JsonValue) -> Result<(), Error> {
	const SCALE: f32 = SPATIUM20;
	for (k, v) in data.entries() {
		match k {
			"stemDownNW" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				// Converting to [0, 1] range with 64 since its the base fmt
				sym.stem_down_nw = Point2F::new(4.0 * DPI_F * x, 4.0 * DPI_F * -y);
			},
			"stemUpSE" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.stem_up_se = Point2F::new(4.0 * DPI_F * x, 4.0 * DPI_F * -y);
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