mod window;
mod painter;

use crate::window::start_window;
use musescore::*;
use musescore::font::{FontMapping, SymName};
use std::path::{PathBuf};
use musescore::drawing::*;
use crate::painter::PfPainter;
use pathfinder_geometry::vector::Vector2I;
use musescore::score::{Score, Note, NoteRenderer, Renderer, RendererState, Element, NoteheadType};

pub fn main() {
	start_window(Vector2I::new(640, 480), "Musescore Demo", draw);
}

pub fn draw(painter: &mut PfPainter) {
	let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/gootville");
	let config = FontMapping::load(&config).unwrap();
	let font = font::load(&path, "gootville.otf", &config).expect("Font must load!");

	let mut state = RendererState::new();
	painter.set_score_font(font.clone());

	let score = Score::new(font.clone());

	let note = Note::new(score);
	note.with_mut(|mut e| {
		e.set_pos(Point2F::new(300., 200.));
		e.set_scale(3.);
	});

	NoteRenderer::layout(note.clone());
	NoteRenderer::render(note.clone(), &mut state, painter);

	let sym = font.sym(SymName::Flag16thUp);
	let sym_char = sym.get_char().expect("Should be a valid character");

	let path = Path::new()
		.set_line_width(6.)
		.move_to(Vec2F::new(50.0, 140.0))
		.add_segment(Segment::Line(Vec2F::new(150.0, 60.0)))
		.add_segment(Segment::Line(Vec2F::new(250.0, 140.0)));

	painter.draw(path.into());

	painter.draw(Symbol::new(
		SymName::NoteheadBlack,
		sym_char,
		Size2F::new(128., 64.),
		Point2F::new(50., 50.)
	).into());
}