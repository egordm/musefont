use crate::score::{Renderer, Note, RendererState, El, ScoreElement, Element};
use crate::drawing::{Painter, PainterRef};
use crate::{Size2F, drawing};
use crate::score::PropertyId::Color;

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
		e.with(|e| {
			let nh_char = e.score().font().sym(e.cached_notehead_sym).get_char().expect("Expected valid font");
			let spatium = e.score().spatium();
			painter.draw(drawing::Symbol::new(
				e.cached_notehead_sym,
				nh_char,
				Size2F::new(e.scale() * spatium, e.scale() * spatium),
				e.pos()
			).into());

			if state.debug() {
				painter.set_color(crate::COLOR_GREEN);
				let a = e.bbox().clone();
				painter.draw(drawing::Instruction::Rect(e.bbox().translate(e.pos().to_vector()), 1.));
				painter.set_color(crate::COLOR_BLUE);
				painter.draw(drawing::Instruction::Point(e.pos(), 2.));
				painter.set_color(crate::COLOR_RED);
				painter.draw(drawing::Instruction::Point(e.stem_up_se() + e.pos().to_vector(), 2.));
				painter.draw(drawing::Instruction::Point(e.stem_down_nw() + e.pos().to_vector(), 2.));
				painter.set_color(crate::COLOR_BLACK);
			}
		})
	}
}

// TODO: create a mock painter and write some draw tests