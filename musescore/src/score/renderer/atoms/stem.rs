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

			if let Some(c) = e.chord() {
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
			let line = LineF::new(Point2F::new(0., y1), Point2F::new(0., l.0));
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

impl StemRenderer {
	/// Get the default stem length for given chord
	fn min_abs_stem_len(e: &El<Chord>) -> f32 {
		0.0 // TODO: tremolo
	}

	/// Get the default stem length for given chord
	fn default_stem_len(er: &El<Chord>) -> Spatium {
		er.with(|e| {
			let hook_type = e.duration_type().hook_type();
			let (ul, dl) = (e.up_line().value().0 as i32, e.down_line().value().0 as i32);
			let line_distance = e.staff().with_d(|st| st.line_distance(&e.time()), Spatium(1.));
			let mut shorten_stem = e.style().value_bool(StyleName::ShortenStem);

			if hook_type.index() >= 2 || e.tremolo().is_some() {
				shorten_stem = true;
			}

			let progression = e.style().value_spatium(StyleName::ShortStemProgression);
			let mut shortest = e.style().value_spatium(StyleName::ShortestStem).0;
			if hook_type != HookType::None {
				shortest = shortest.min(if e.up() { 3.0 } else { 3.5 });
			}

			let mut normal_stem_len = if e.small() { 2.5 } else { 3.5 }
				+ hook_adjustment(&e.font().name(), hook_type, e.up(), e.small());
			if hook_type != HookType::None && e.up() && e.duration_type().dots() != 0 {
				// Avoid collision of dot with the hook
				if (ul & 1) != 0 { normal_stem_len += 0.5; }
				else { shorten_stem = false; }
			}

			let mut stem_len;
			if e.is_grace() {
				// grace notes stems are not subject to normal stem rules
				stem_len = (ul - dl) as f32 * 0.5;
				stem_len *= normal_stem_len * e.style().value_f32(StyleName::GraceNoteMag);
				if e.up() { stem_len *= -1. }
			} else {
				let staff_height = e.staff().with_d(|st| st.lines(&e.time()).value().0, 4.) as f32 * line_distance.0;
				let dn_mirror = e.down_note().with_d(|dn| dn.mirror(), false);
				if e.up() { // stem up
					let dy = dl as f32 * 0.5;                        // note-side vert. pos.
					let mut sel = ul as f32 * 0.5 - normal_stem_len; // stem end vert. pos

					// if stem ends above top line (with some exceptions), shorten it
					if shorten_stem && sel < 0.0 && (hook_type == HookType::None || !dn_mirror) {
						sel -= sel * progression.0;
					}
					sel = sel.min(staff_height * 0.5); // if stem ends below ('>') staff mid position, stretch it to mid position
					stem_len = sel - dy;  // actual stem length
					if -stem_len < shortest { stem_len = -shortest } // is stem too short lengthen it to shortest possible length
				} else {
					let uy = ul as f32 * 0.5;                        // note-side vert. pos.
					let mut sel = dl as f32 * 0.5 + normal_stem_len; // stem end vert. pos.

					// if stem ends below bottom line (with some exceptions), shorten it
					if shorten_stem && sel > staff_height && (hook_type == HookType::None || dn_mirror) {
						sel -= (sel - staff_height) * progression.0;
					}
					// if stem ends above ('<') staff mid position, stretch it to mid position
					sel = sel.max(staff_height * 0.5);
					stem_len = (sel - uy).max(shortest);  // actual stem length; lengthen it to shortest possible position
				}
			}

			// TODO: adjust for tremolo

			let sign = if e.up() { -1.0 } else { 1.0 };
			let mut stem_len_points = stem_len * e.spatium();
			let min_abs_len = Self::min_abs_stem_len(&er);
			if sign * stem_len_points < min_abs_len { stem_len_points = sign * min_abs_len }

			return Spatium(-stem_len_points);
		})
	}

	/// Layout _stem and _stemSlash
	/// Called before layout spacing of notes.
	/// Create stem if necessary.
	/// Should be called in the beam
	pub fn layout_chord_stem(e: &El<Chord>) {
		let has_stem = e.with(|e| {
			let stemless_measure = e.measure().with_d(|m| m.stemless(e.staff_id()), false);
			let stemless_staff= e.staff().with_d(|s| s.staff_type(&e.time()).stemless(), false) && false; // TODO only used for tabbed
			e.duration_type().has_stem() && !(e.no_stem() || stemless_measure || stemless_staff)
		});

		if has_stem {
			if e.borrow_el().stem().is_none() {
				let stem = Stem::new(e.borrow_el().score().clone());
				stem.with_mut(|mut stem| {
					stem.set_parent(Some(ElementRef::from(e.clone()).downgrade()));
					stem.set_generated(true);
				});
			}

			// Check if a slash is needed.
			if e.with(|ec| { ec.note_type() == NoteType::Acciaccatura
				&& !(ec.beam().map(|b| b.borrow_el().elements().next() != Some(&ChordRef::Chord(e.clone()))).unwrap_or(false)) }) {
				if e.borrow_el().stem_slash().is_none() {
					e.borrow_mut_el().add(StemSlash::new(e.borrow_el().score().clone()).into())
				}
			} else if let Some(slash) = e.borrow_el().stem_slash() {
				e.borrow_mut_el().remove(&ElementRef::from(slash.clone()))
			}

			if let Some(stem) = e.borrow_el().stem() {
				stem.with_mut(|mut stem| {
					let stem_width5 = stem.line_width() * 0.5 * e.borrow_el().scale();
					let pos = e.with(|e| {
						let x = e.stem_pos_x() + if e.up() { -stem_width5 } else { stem_width5 };
						Point2F::new(x, stem.pos().y)
					});
					stem.set_pos(pos);
					stem.set_len(Self::default_stem_len(&e))
				})
			}
		} else {
			// remove stem and stemslash
			e.with_mut(|mut e| {
				e.set_stem(None);
				e.set_stem_slash(None);
			})
		}
	}
}
