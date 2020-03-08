use crate::score::{Renderer, Note, RendererState, El, ScoreElement, Element};
use crate::drawing::{Painter, PainterRef};
use crate::{Size2F, drawing};

pub struct NoteRenderer {

}

impl Renderer<Note> for NoteRenderer {
	fn layout(e: El<Note>) {
		// Two types of borrows are  redundant in this case. But serve as an example of two phases
		let r = e.borrow_el();
		let nh = r.notehead();
		let scale = Size2F::new(r.scale(), r.scale());
		let bb = r.score().font().bbox(nh, &scale);

		drop(r); // moves imm ref out of scope
		e.with_mut(|mut e| {
			e.set_bbox(bb);
			e.cached_notehead_sym = nh;
		});
	}

	fn render(e: El<Note>, state: &mut RendererState, painter: PainterRef) {
		let r = e.borrow_el();
		let nh_char = r.score().font().sym(r.cached_notehead_sym).get_char().expect("Expected valid font");
		let spatium = r.score().spatium();
		painter.draw(drawing::Symbol::new(
			r.cached_notehead_sym,
			nh_char,
			Size2F::new(r.scale() * spatium, r.scale() * spatium),
			r.pos()
		).into())
	}
}