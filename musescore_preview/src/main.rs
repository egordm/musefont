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
	let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/gootville");
	let config = font::FontMapping::load(&config).unwrap();
	let font = font::load(&path, "gootville.otf", &config).expect("Font must load!");

	let mut state = RendererState::new();
	state.set_debug(true);
	painter.set_score_font(font.clone());
	painter.set_dpi(96.);
	painter.set_scale(6.);

	let score = Score::new(font.clone());

	let chord = Chord::new(score.clone());
	chord.with_mut(|mut e| {
		e.set_pos(Point2F::new(200., 200.))
	});

	let note = Note::new(score.clone());
	chord.borrow_mut_el().add(note.clone().into());

	let stem = Stem::new(score.clone());
	chord.borrow_mut_el().add(stem.clone().into());

	let hook = Hook::new(score.clone());
	hook.with_mut(|mut e| {
		e.set_hook_type(HookType::Flag64thUp);
	});


/*	NoteRenderer::layout(note.clone());
	NoteRenderer::render(note.clone(), &mut state, painter);
	StemRenderer::layout(stem.clone());
	StemRenderer::render(stem.clone(), &mut state, painter);
	HookRenderer::layout(hook.clone());
	HookRenderer::render(hook.clone(), &mut state, painter);*/

	ChordRenderer::layout(chord.clone());
	ChordRenderer::render(chord.clone(), &mut state, painter);
}