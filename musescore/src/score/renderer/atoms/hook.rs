use crate::font::SymName;
use crate::score::*;
use crate::drawing::PainterRef;
use crate::{Size2F, drawing, Vec2F};

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
			if sym != SymName::NoSym {
				painter.draw(drawing::Symbol::from_font(
					&*e.font(),
					sym,
					e.pos(),
					Size2F::new(e.scale(), e.scale())
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

impl HookRenderer {
	/// Called before layouting the hook.
	/// Hook is created when it should exist
	pub fn layout_before(e: &El<Chord>) {
		let hook_type = e.with(|e| {
			let hook_type = e.duration_type().hook_type();
			if hook_type != HookType::None
				&& !e.no_stem()
				&& !e.measure().with_d(|m| m.stemless(e.staff_id()), false) {
				hook_type.direction(e.up())
			} else {
				HookType::None
			}
		});

		// Create a hook if needed or delete the hook if it is empty
		if hook_type != HookType::None {
			if e.borrow_el().hook().is_none()  {
				let hook = Hook::new(e.borrow_el().score().clone());
				hook.with_mut(|mut hook| {
					hook.set_generated(true);
				});
				e.borrow_mut_el().add(hook.into());
			}
		} else if let Some(hook) = e.with(|e| e.hook().cloned()) {
			e.borrow_mut_el().remove(&hook.into())
		}

		// Set hook type
		if let Some(hook) = e.borrow_el().hook() {
			hook.with_mut(|mut hook| {
				hook.set_hook_type(hook_type)
			});
		}
	}

	/// Called after the stem and the hook are done with their layout
	pub fn layout_after(e: &El<Chord>) {
		e.with(|e| {
			if let (Some(stem), Some(hook)) = (e.stem(), e.hook()) {
				// Position the hook properly
				let p = stem.with(|stem| {
					let y_offset = hook.with(|hook| if e.up() { hook.bbox().min_y() } else { hook.bbox().max_y() });
					return stem.hook_pos() - Vec2F::new(stem.width(), y_offset);
				});
				hook.borrow_mut_el().set_pos(p);
			}
		});
	}
}

pub fn hook_len_adjustment(_font: &str, hooks: HookType, up: bool, small: bool) -> Spatium {
	let hooks = hooks.index();
	let fallback = hooks > 5; // && use fallback font

	let font = "Emmentaler"; // TODO: dont hardcode
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
