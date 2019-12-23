#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

pub mod types;
pub mod constants;
pub mod structs;
pub mod load;

pub use types::*;
pub use constants::*;
pub use structs::*;
pub use load::*;


#[cfg(test)]
mod test {
	use std::path::PathBuf;
	use crate::load::load;
	use crate::*;

	#[test]
	pub fn test_load() {
		let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/gootville");
		let filename = "gootville.otf";
		let config = FontConfig::new(&config).unwrap();
		let mut font = load(&path, filename, &config).unwrap();

		pretty_print(&mut font, SymId::NoteheadBlack);
		pretty_print(&mut font, SymId::Rest32nd);
	}

	pub fn pretty_print(font: &mut ScoreFont, sym_id: SymId) {
		let pixels = font.pixmap(sym_id, &SIZE_ONE, 64., RasterizationOptions::GrayscaleAa, Format::A8).unwrap().canvas();

		let mut res = String::new();
		for y in 0..pixels.size.height as usize {
			for x in 0..pixels.size.width as usize {
				let idx = x + y * pixels.stride;
				if pixels.pixels[idx] > 0 {
					res.push('#');
				} else {
					res.push('.');
				}
			}
			res.push('\n');
		}
		println!("{}", res);

	}
}