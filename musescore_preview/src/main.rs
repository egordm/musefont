mod window;
mod painter;

use crate::window::start_window;
use musescore::*;
use musescore::font;
use musescore::drawing;
use musescore::drawing::Painter;
use musescore::score::*;
use pathfinder_geometry::vector::Vector2I;
use std::path::{PathBuf};
use crate::painter::PfPainter;
use musescore::constants::{SPATIUM20, DPI_F};

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

	let score = Score::new(font.clone());

	let chord = Chord::new(score.clone());
	chord.with_mut(|mut e| {
		e.set_scale(6.);
	});

	let note = Note::new(score.clone());
	note.with_mut(|mut e| {
		e.set_pos(Point2F::new(300., 200.));
		e.set_scale(6.);
	});
	chord.borrow_mut_el().add(note.clone().into());

	let stem = Stem::new(score.clone());
	stem.with_mut(|mut e| {
		e.set_scale(6.);
		e.set_line_width(score.style().pvalue(StyleName::StemWidth) / 4.); // TODO: why must i divide by 4
		let line_width = e.line_width() * e.scale() * 0.5;
		e.set_pos(Point2F::new(300. + note.borrow_mut_el().head_width() - line_width, 300.));
		e.set_user_len(6. * score.spatium());
	});
	chord.borrow_mut_el().add(stem.clone().into());

	let hook = Hook::new(score.clone());
	hook.with_mut(|mut e| {
		e.set_scale(6.);
		e.set_hook_type(HookType::Flag64thUp);
		e.set_pos(Point2F::new(360., 100.));
	});


	NoteRenderer::layout(note.clone());
	NoteRenderer::render(note.clone(), &mut state, painter);
	StemRenderer::layout(stem.clone());
	StemRenderer::render(stem.clone(), &mut state, painter);
	HookRenderer::layout(hook.clone());
	HookRenderer::render(hook.clone(), &mut state, painter);

	let sym = font.sym(font::SymName::Flag16thUp);
	let sym_char = sym.get_char().expect("Should be a valid character");

	/*painter.draw(drawing::Symbol::new(
		font::SymName::NoteheadBlack,
		sym_char,
		Size2F::new(128., 64.),
		Point2F::new(50., 50.)
	).into());*/
}