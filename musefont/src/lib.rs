#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

pub mod types;
pub mod constants;
pub mod structs;
pub mod load;
pub mod notaton;

pub use types::*;
pub use constants::*;
pub use structs::*;
pub use load::*;
pub use notaton::*;


#[cfg(test)]
mod test {
	use std::path::PathBuf;
	use crate::load::load_font;
	use crate::*;

	#[test]
	pub fn test_load() {
		let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/gootville");
		let filename = "gootville.otf";
		let config = FontConfig::new(&config).unwrap();
		let mut font = load_font(&path, filename, &config).unwrap();

		let aa = font.sym(SymIdent::NoteheadBlack as u32);
		let uu = aa.stem_down_nw();
		let ua = aa.stem_up_se();

		pretty_print(&mut font, SymIdent::NoteheadBlack);
		pretty_print(&mut font, SymIdent::Rest32nd);
	}

	pub fn pretty_print(font: &mut ScoreFont, sym_id: SymIdent) {
		let mut a = font.sym(sym_id as SymId).stem_down_nw() * 0.7;
		let mut b = font.sym(sym_id as SymId).stem_up_se() * 0.7;
		//a.y += -font.sym(sym_id as SymId).bbox().origin.y * 0.7;
		//b.y += -font.sym(sym_id as SymId).bbox().origin.y * 0.7;

		let pixels = font.pixmap(sym_id as SymId, &(SIZE_ONE * 0.7), RasterizationOptions::GrayscaleAa, Format::A8).unwrap().canvas();

		let ai = (a.y.floor() as usize) * pixels.stride + (a.x.floor() as usize);
		let bi = (b.y.floor() as usize) * pixels.stride + (b.x.floor() as usize);

		let mut res = String::new();
		for y in 0..pixels.size.height as usize {
			for x in 0..pixels.size.width as usize {
				let idx = x + y * pixels.stride;
				if ai == idx {
					res.push('X');
				} else if bi == idx {
					res.push('O');
				} else if pixels.pixels[idx] > 0 {
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