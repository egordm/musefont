use crate::font::SymName;
use crate::score::*;
use crate::drawing::PainterRef;
use crate::{Size2F, drawing, Point2F};

pub struct HookRenderer {}

impl Renderer<Hook> for HookRenderer {
	fn layout(e: El<Hook>) {
		e.with_mut(|mut e| {
			let scale = e.scale();
			let bb = e.score().font().bbox(
				e.sym(),
				&Size2F::new(scale, scale)
			);
			e.set_bbox(bb);
		});
	}

	fn render(e: El<Hook>, state: &mut RendererState, painter: PainterRef) {
		e.with(|e| {
			let sym = e.sym();
			let spatium = e.score().spatium();
			if sym != SymName::NoSym {
				painter.draw(drawing::Symbol::from_font(
					&*e.font(),
					sym,
					e.pos(),
					Size2F::new(e.scale() * spatium, e.scale() * spatium)
				).into());
			}

			if state.debug() {
				painter.set_color(crate::COLOR_GREEN);
				painter.draw(drawing::Instruction::Rect(e.bbox().translate(e.pos().to_vector()), 1.));
				painter.set_color(crate::COLOR_BLUE);
				painter.draw(drawing::Instruction::Point(e.pos(), 2.));
				painter.set_color(crate::COLOR_BLACK);
			}
		});
	}
}