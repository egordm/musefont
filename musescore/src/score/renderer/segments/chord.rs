use crate::font::SymName;
use crate::score::*;
use crate::drawing::*;
use crate::{Point2F, drawing, Vec2F};
use crate::score::PropertyId::P1;

pub struct ChordRenderer {

}

impl ChordRenderer {
	/// Layout grace notes such that they are processed left to right
	fn layout_grace_notes(e: El<Chord>) {
		// Layout grace notes before
		for (i, c) in e.borrow_el().grace_notes().iter().enumerate() {
			c.borrow_mut_el().set_grace_index(i as i32);
			if c.borrow_el().is_grace_before() {
				ChordRenderer::layout(c.clone());
			}
		}

		// Layout grace notes after
		for (i, c) in e.borrow_el().grace_notes().iter().enumerate() {
			if c.borrow_el().is_grace_before() {
				ChordRenderer::layout(c.clone());
			}
		}
	}

	fn add_ledger_lines(e: El<Chord>) {
		// TODO: add them
		// unimplemented!()
	}
}

impl Renderer<Chord> for ChordRenderer {
	fn layout(e: El<Chord>) {
		if e.borrow_el().notes().is_empty() { return; }

		ChordRenderer::layout_grace_notes(e.clone());
		// TODO: Calculate the bounds and spacings

		// Remove hook if has beam
		if let Some(hook) = e.borrow_el().hook() {
			if e.borrow_el().beam().is_some() {
				e.borrow_mut_el().remove_element(&hook.clone().into());
			}
		}

		// Layout the notes
		for note in e.borrow_el().notes() {
			NoteRenderer::layout(note.clone());
			// TODO: calculate the real note bound
		}

		// TODO: correct spacing with gracenotes.
		// TODO: layout all children. Need to add it to trait as assoc (use spacelw and spacerw)
		// TODO: move stem rendeing somewhere else
		StemRenderer::layout_chord_stem(&e);
		if let Some(stem) = e.borrow_el().stem() {
			StemRenderer::layout(stem.clone());

			// Layout the hook
			if let Some(hook) = e.borrow_el().hook() {
				HookRenderer::layout(hook.clone());
				HookRenderer::layout_chord_hook(&e);
			}
		}

		// Create ledger lines
		ChordRenderer::add_ledger_lines(e.clone());

		// That should be it. Now check if it works?
		// TODO: add layoutStem1 (should be in different place but yeah who cares)
		//unimplemented!()
	}

	fn render(e: El<Chord>, state: &mut RendererState, painter: PainterRef) {
		// unimplemented!()
		// Draw children or what?

		e.with(|e| {
			painter.translate(e.pos().to_vector());

			if let Some(stem) = e.stem() {
				StemRenderer::render(stem.clone(), state, painter);

				if let Some(hook) = e.hook() {
					HookRenderer::render(hook.clone(), state, painter);
				}
			}

			for note in e.notes() {
				NoteRenderer::render(note.clone(), state, painter);
			}

			if state.debug() {
				// painter.set_color(crate::COLOR_GREEN);
				// painter.draw(drawing::Instruction::Rect(e.bbox().translate(e.pos().to_vector()), 1.));
				painter.set_color(crate::COLOR_BLUE);
				painter.draw(drawing::Instruction::Point(Point2F::new(0., 0.), 2.));
				painter.set_color(crate::COLOR_RED);
			}

			painter.translate(-e.pos().to_vector());
		});
	}
}