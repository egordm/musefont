use crate::font::SymName;
use crate::score::*;
use crate::drawing::*;
use crate::Point2F;

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

		let dot_pos = e.borrow_el().dot_pos_x();

		// Remove hook if has beam
		if let Some(hook) = e.borrow_el().hook() {
			if e.borrow_el().beam().is_some() {
				e.borrow_mut_el().remove_element(&ElementRef::from(hook.clone()));
			}
		}

		let (space_lw, space_rm) = e.with(|e| {
			let spatium = e.spatium();

			let mag = e.staff().with_d(|s| s.mag(&e.time()), 1.);
			let dot_note_distance = e.score().style().value_p(StyleName::DotNoteDistance) * mag;
			let min_note_distance = e.score().style().value_p(StyleName::MinNoteDistance) * mag;
			let min_tie_length = e.score().style().value_p(StyleName::MinTieLength) * mag;

			let grace_mag = e.score().style().value_f32(StyleName::GraceNoteMag);
			let chord_x = if e.note_type() == NoteType::Normal { e.ipos().x } else { 0. };

			let mut lll = 0.0f32; // space to leave at left of chord
			let mut rrr = 0.0f32; // space to leave at right of chord
			let mut lhead = 0.0f32; // amount of notehead to left of chord origin
			let upnote = e.up_note();

			for note in e.notes() {
				NoteRenderer::layout(note.clone());

				note.with(|note| {
					let x1 = note.pos().x + chord_x;
					let x2 = x1 + note.head_width();
					lll = lll.max(-x1);
					rrr = rrr.max(x2);
					lhead = lhead.max(-x1); // track amount of space due to notehead only

					// Use accidental
					if let Some(accidental) = note.accidental() {
						if note.fixed() {
							// convert x position of accidental to segment coordinate system
							let mut x = accidental.borrow_el().pos().x + note.pos().x + chord_x;
							/* distance from accidental to note already taken into account
							   but here perhaps we create more padding in *front* of accidental? */
							x -= e.score().style().value_p(StyleName::AccidentalDistance) * mag;
							lll = lll.max(-x);
						}
					}

					/* allow extra space for shortened ties this code must be kept synchronized
					   with the tie positioning code in Tie::slurPos()
					   but the allocation of space needs to be performed here */
					if let Some(tie) = note.tie_back() {
						// TODO: tie calculate direction
					}
				});
			}

			// Layout arpeggio
			if let Some(arpeggio) = e.arpeggio().map(ElWeak::upgrade).flatten() {
				let arpeggio_distance = e.score().style().value_p(StyleName::ArpeggioNoteDistance) * mag;
				// TODO: layout
				lll += arpeggio.borrow_el().width() + arpeggio_distance + chord_x;
				let y1 = upnote.with(|u| u.pos().y - u.head_height() * 0.5).unwrap();
				arpeggio.with_mut(|mut a| {
					a.set_height(0.);
					a.set_pos(Point2F::new(-lll, y1));
				});
			}

			// Layout glissando
			if e.ends_glissando() {
				// if not at beginning of measure or there are graces before
				if !e.time().is_zero()
					|| e.grace_notes().iter().any(|n| n.borrow_el().is_grace_after()) {
					lll += spatium * 0.5 + min_tie_length;
					// special case of system-initial glissando final note is handled in Glissando::layout() itself
				}
			}

			// Layout dots
			if e.dots() > 0 {
				let mut x = dot_pos + dot_note_distance + (e.dots() as f32 - 1.)
					* e.score().style().value_p(StyleName::DotDotDistance) * mag;
				x += e.font().width(SymName::AugmentationDot, e.scale());
				rrr = rrr.max(x);
			}

			if let Some(hook) = e.hook() {
				HookRenderer::layout(hook.clone());
				if e.up() && e.stem().is_some() {
					let x = hook.borrow_el().bbox().max_x() + e.stem().unwrap().borrow_el().hook_pos().x + chord_x;
					rrr = rrr.max(x);
				}
			}

			(lll, rrr)
		});

		// TODO: correct spacing with gracenotes.
		// TODO: layout all children. Need to add it to trait as assoc (use spacelw and spacerw)
		// TODO: move stem rendeing somewhere else
		StemRenderer::layout_chord_stem(&e);
		if let Some(stem) = e.borrow_el().stem() {
			StemRenderer::layout(stem.clone())
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
			if let Some(stem) = e.stem() {
				StemRenderer::render(stem.clone(), state, painter);
			}

			for note in e.notes() {
				NoteRenderer::layout(note.clone());
			}
		});
	}
}