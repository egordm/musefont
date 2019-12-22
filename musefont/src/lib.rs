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

/*
mod test {
	use font_kit::source::SystemSource;
	use font_kit::canvas::{Format, RasterizationOptions, Canvas};
	use font_kit::hinting::HintingOptions;

	pub fn aaaa() {
		let font = SystemSource::new()
			.select_by_postscript_name("ArialMT")
			.unwrap()
			.load()
			.unwrap();

		let glyph_id = font.glyph_for_char('A').unwrap();
		let mut canvas = Canvas::new(&Size2D::new(32, 32), Format::A8);

		font.rasterize_glyph(
			&mut canvas,
			glyph_id,
			32.0,
			&Point2D::new(0.0, 32.0),
			HintingOptions::None,
			RasterizationOptions::GrayscaleAa,
		)
			.unwrap();
	}
}*/
