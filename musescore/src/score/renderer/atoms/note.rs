use crate::score::{Renderer, Note, RendererState, El, ScoreElement, Element};
use crate::drawing::{Painter, PainterRef};
use crate::{Size2F, drawing};

pub struct NoteRenderer {

}

impl Renderer<Note> for NoteRenderer {
	fn layout(e: El<Note>) {
		let r = e.borrow_el();
		let nh = r.notehead();
		let scale = Size2F::new(r.scale(), r.scale());
		let bb = r.score().font().bbox(nh, &scale);

		let mut r = e.borrow_mut_el();
		r.set_bbox(bb);
		r.cached_notehead_sym = nh;
	}

	fn render(e: El<Note>, state: &mut RendererState, painter: &mut PainterRef) {
		let r = e.borrow_el();
		let nh_char = r.score().font().sym(r.cached_notehead_sym).get_char().expect("Expected valid font");
		painter.draw(drawing::Symbol::new(
			r.cached_notehead_sym,
			nh_char,
			Size2F::new(r.scale(), r.scale()),
			r.pos()
		).into())
	}
}