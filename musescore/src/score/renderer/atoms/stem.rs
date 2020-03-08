use crate::*;
use crate::score::*;
use crate::drawing::{PainterRef};
use crate::{Point2F, LineF, RectF};

pub struct StemRenderer {

}

impl Renderer<Stem> for StemRenderer {
	fn layout(e: El<Stem>) {
		e.with_mut(|mut e| {
			let up = e.up();
			let vscale = if up { -1. } else { 1. };
			let mut l = (e.len() + e.user_len()) * vscale;
			let mut y1 = 0.;

			if let Some(ChordRef::Chord(c)) = e.chord() {
				c.with(|chord| {
					e.set_scale(chord.scale());
					if let Some(note) = if up { chord.down_note() } else { chord.up_note() } {
						y1 += if up { note.borrow_el().stem_up_se().y } else { note.borrow_el().stem_down_nw().y };
						let pos = Point2F::new(e.x(), note.borrow_el().y());
						e.set_pos(pos);
					}
				})
			}

			let lw5 = e.line_width() * 0.5 * e.scale();
			let line = LineF::new(Point2F::new(0., y1), Point2F::new(0., l));
			e.set_line(line.clone());
			let bbox = e.line().rect().adjust(Point2F::new(-lw5, -lw5), Point2F::new(lw5, lw5));
			e.set_bbox(bbox);
		});
	}

	fn render(e: El<Stem>, state: &mut RendererState, painter: PainterRef) {
		e.with(|e| {
			let line = e.line().clone() + e.pos().to_vector();
			let path = drawing::Path::new()
				.move_to(line.p1.to_vector())
				.add_segment(drawing::Segment::Line(line.p2.to_vector()))
				.set_line_width(e.line_width() * e.scale())
				.set_line_cap(drawing::LineCap::Round);
			painter.draw(path.into());

			if state.debug() {
				painter.set_color(crate::COLOR_GREEN);
				painter.draw(drawing::Instruction::Rect(e.bbox().translate(e.pos().to_vector()), 1.));
				painter.set_color(crate::COLOR_BLUE);
				painter.draw(drawing::Instruction::Point(e.pos(), 2.));
				painter.set_color(crate::COLOR_RED);
				painter.draw(drawing::Instruction::Point(line.p1, 2.));
				painter.draw(drawing::Instruction::Point(line.p2, 2.));
				painter.set_color(crate::COLOR_BLACK);
			}
		});
	}
}