use std::{collections::HashMap, path::Path};
use num_traits::FromPrimitive;
use crate::*;

type SymLut = HashMap<&'static str, SymId>;

pub struct FontConfig {
	pub(crate) sym_lut: SymLut,
	pub(crate) sym_codes: Vec<u32>,
}

type Error = FontLoadingError;

impl FontConfig {
	pub fn new(path: &Path) -> Result<Self, Error> {
		let sym_lut = sym_lut();
		let mut sym_codes = vec![0; SYMBOL_COUNT];

		let meta_path = path.join("glyphnames.json");
		let meta_str = std::fs::read_to_string(&meta_path).map_err(Error::IO)?;
		let meta = json::parse(&meta_str).map_err(Error::Json)?;

		for (i, name) in SYMBOL_NAMES.iter().cloned().enumerate() {
			let code = meta[name]["codepoint"].as_str().and_then(|s| u32::from_str_radix(&s[2..], 16).ok()).unwrap_or(0);
			sym_codes[i] = code;
		}

		Ok(Self { sym_lut, sym_codes})
	}
}

fn sym_lut() -> SymLut {
	SYMBOL_NAMES.iter().cloned()
		.zip(
			(0..SYMBOL_COUNT).into_iter()
				.map(|x| SymId::from_usize(x).unwrap())
		).collect()
}
