mod window;
mod painter;

use crate::window::start_window;
use musescore::*;
use musescore::font;
use musescore::drawing::Painter;
use musescore::score::*;
use pathfinder_geometry::vector::Vector2I;
use std::path::{PathBuf};
use crate::painter::PfPainter;

pub fn main() {
	start_window(Vector2I::new(640, 480), "Musescore Demo", draw);
}

pub fn draw(painter: &mut PfPainter) {
	// TODO: font specific overrides
	let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/mscore");
	let config = font::FontMapping::load(&config).unwrap();
	let font = font::load(&path, "mscore.ttf", &config).expect("Font must load!");

	let mut state = RendererState::new();
	state.set_debug(false);
	painter.set_score_font(font.clone());
	painter.set_dpi(96.);
	painter.set_scale(6.);

	let score = Score::new(font.clone());

	let chord = Chord::new(score.clone());
	chord.with_mut(|mut e| {
		e.set_pos(Point2F::new(100., 100.))
	});

	let note = Note::new(score.clone());
	chord.borrow_mut_el().add(note.clone().into());

	let stem = Stem::new(score.clone());
	stem.with_mut(|mut e| {
		let line_width = e.style().value_spatium(StyleName::StemWidth).points(e.spatium());
		e.set_line_width(line_width);
	});
	chord.borrow_mut_el().add(stem.clone().into());

	let hook = Hook::new(score.clone());
	hook.with_mut(|mut e| {
		e.set_hook_type(HookType::Flag8thUp);
	});
	chord.borrow_mut_el().add(hook.clone().into());


	ChordRenderer::layout(chord.clone());
	ChordRenderer::render(chord.clone(), &mut state, painter);
}