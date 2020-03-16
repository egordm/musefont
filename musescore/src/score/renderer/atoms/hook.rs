use crate::font::SymName;
use crate::score::*;
use crate::drawing::PainterRef;
use crate::{Size2F, drawing, Point2F};

pub struct HookRenderer {}

impl Renderer<Hook> for HookRenderer {
	fn layout(e: El<Hook>) {
		e.with_mut(|mut e| {
			let scale = e.scale();
			let bb = e.score().font().bbox(e.sym(), &(scale, scale).into());
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

pub fn hook_len_adjustment(font: &str, hooks: HookType, up: bool, small: bool) -> Spatium {
	let hooks = hooks.index();
	let fallback = hooks > 5; // && use fallback font

	let font = "Gonville";
	let value = match (font, fallback) {
		("Emmentaler", false) => {
			if up {
				if hooks > 2 { (hooks as f32 - 2.) * (if small { 0.75 } else { 1. }) }
				else { 0. }
			} else {
				if hooks == 3 { if small { 0.75 } else { 1. } }
				else if hooks > 3 { (hooks as f32 - 2.) * (if small { 0.5 } else { 0.75 }) }
				else { 0. }
			}
		},
		("Gonville", false) => {
			if up {
				if hooks > 2 { (hooks as f32 - 2.) * (if small { 0.5 } else { 0.75 }) }
				else { 0. }
			} else {
				if hooks > 1 { (hooks as f32 - 1.) * (if small { 0.5 } else { 0.75 }) }
				else { 0. }
			}
		}
		("MuseJazz", _) => {
			if hooks > 2 { (hooks as f32 - 2.) * (if small { 0.75 } else { 1. }) }
			else { 0. }
		}
		_ => {
			if hooks > 2 { (hooks as f32 - 2.) * (if small { 0.5 } else { 0.75 }) }
			else { 0. }
		}
	};
	Spatium(value)
}
