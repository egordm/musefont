use std::path::{PathBuf};
use crate::font;
use crate::score::*;

pub(crate) fn setup_font() -> font::ScoreFont {
	let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/mscore");
	let config = font::FontMapping::load(&config).unwrap();
	font::load(&path, "mscore.ttf", &config).expect("Font must load!")
}

pub(crate) fn setup_score() -> Score {
	let font = setup_font();
	Score::new(font)
}

pub(crate) fn setup_painter() {
	// TODO: add mock painter
}