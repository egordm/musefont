use crate::score::*;
use crate::drawing::{Painter, PainterRef};
use crate::{Size2F, drawing, Point2F};
use crate::score::PropertyId::Color;

pub struct NoteRenderer {

}

impl NoteRenderer {
	/// called after final position of note is set
	pub fn layout_after(e: El<Note>) {
		e.with(|e| {
			let dots = e.chord().map(|c| c.borrow_el().dots()).unwrap_or_default();
			if dots > 0 {
				let d = e.score().point(e.style().value_spatium(StyleName::DotNoteDistance)) * e.scale();
				let dd = e.score().point(e.style().value_spatium(StyleName::DotDotDistance)) * e.scale();
				let x = e.chord().map(|c| c.with(|c|
					c.dot_pos_x() - e.pos().x - c.pos().x
				)).unwrap_or(0.);
				let mut xx = x + d;
				for dot in e.dots() {
					let pos = dot.borrow_el().pos();
					dot.borrow_mut_el().set_pos(Point2F::new(xx, pos.y));
					xx += dd;
				}
			}
		});

		// TODO: Layout attached elements
	}
}

impl Renderer<Note> for NoteRenderer {
	fn layout(e: El<Note>) {
		// Two types of borrows are  redundant in this case. But serve as an example of two phases
		let r = e.borrow_el();
		let nh = r.notehead();
		let scale = Size2F::new(r.scale(), r.scale());
		let bb = r.font().bbox(nh, &scale);

		drop(r); // moves imm ref out of scope
		e.with_mut(|mut e| {
			e.set_bbox(bb);
			e.cached_notehead_sym = nh;
		});
	}

	fn render(e: El<Note>, state: &mut RendererState, painter: PainterRef) {
		e.with(|e| {
			let spatium = e.score().spatium();
			painter.draw(drawing::Symbol::from_font(
				&*e.font(),
				e.cached_notehead_sym,
				e.pos(),
				Size2F::new(e.scale() * spatium, e.scale() * spatium),
			).into());

			if state.debug() {
				painter.set_color(crate::COLOR_GREEN);
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