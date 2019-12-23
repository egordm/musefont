use font_kit::loaders::freetype::Font;
use num_traits::FromPrimitive;
use std::{path::Path, fs::File, collections::HashMap};
use crate::*;

type Error = FontLoadingError;

pub fn load(path: &Path, filename: &str, state: &FontState) -> Result<ScoreFont, Error> {
	let mut font_file = File::open(path.join(filename)).map_err(Error::IO)?;
	let font = Font::from_file(&mut font_file, 0).map_err(Error::Font)?;
	let mut font = ScoreFont::new(font);

	// Compute metrics for all symbols
	for (id, code) in state.sym_codes.iter().cloned().enumerate() {
		compute_metrics(&mut font.symbols[id], code, &font.font)?;
	}

	let mut meta_str = std::fs::read_to_string(path.join("metadata.json")).map_err(Error::IO)?;
	let meta = json::parse(&meta_str).map_err(Error::Json)?;

	let symbol_lut = sym_lut();

	// Load symbol data
	for (ident, v) in meta["glyphsWithAnchors"].entries() {
		let sym_id = symbol_lut.get(ident).cloned().unwrap_or(SymId::NoSym);
		if sym_id == SymId::NoSym { continue; }
		parse_sym(&mut font.symbols[sym_id as usize], v)?;
	}

	// TODO: engravings

	// Load compound data and recalculte the bounding boxes
	for (sym_id, children) in COMPOSED_SYMBOLS.iter().cloned() {
		if !font.symbols[sym_id as usize].is_valid() {
			let bb = font.bounding_box_combined(children.iter().cloned(), &Size2F::new(1., 1.));
			let sym = &mut font.symbols[sym_id as usize];
			sym.compound_ids = children.iter().cloned().collect();
			sym.bbox = bb;

		}
	}

	// TODO: style

	compute_metrics(&mut font.symbols[SymId::Space as usize], 32, &font.font)?;
	Ok(font)
}

fn compute_metrics(sym: &mut Sym, code: u32, font: &Font) -> Result<(), Error> {
	const M: f32 = 640.0 / DPI_F;

	if let Some(char) = std::char::from_u32(code) {
		if let Some(glyph_id) = font.glyph_for_char(char) {
			let bb = font.typographic_bounds(glyph_id).map_err(Error::Glyph)?;
			let bb = RectF::new(
				Point2F::new(bb.origin.x, -(bb.origin.y + bb.size.height)),
				bb.size
			) / M;
			sym.code = code as i32;
			sym.index = glyph_id;
			sym.bbox = bb;
			sym.advance = font.advance(glyph_id).map_err(Error::Glyph)?.x * DPI_F/ 655360.0;
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

#[cfg(test)]
mod test {
	use std::path::PathBuf;
	use crate::load::load;
	use crate::{SymId, FontState};

	#[test]
	pub fn test_load() {
		let path = PathBuf::from("./assets/fonts/gootville");
		let filename = "gootville.otf";
		let state = FontState::new().unwrap();
		let font = load(&path, filename, &state).unwrap();

		let test = font.sym(SymId::NoteheadBlack);
		let i = 0;
	}
}