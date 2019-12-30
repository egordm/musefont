use crate::*;
use crate::notaton::musical_elements::hook::HookType;
use crate::notaton::musical_elements::accidental::Type::SharpThreeArrowsDown;
use std::borrow::Borrow;

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum NoteType {
	Normal = 0,
	Acciaccatura  = 0x1,
	Appoggiatura  = 0x2,       // grace notes
	Grace4        = 0x4,
	Grace16       = 0x8,
	Grace32       = 0x10,
	Grace8After = 0x20,
	Grace16After = 0x40,
	Grace32After = 0x80,
	Invalid       = 0xFF
}

#[derive(Clone, Debug)]
struct Cache {
	stem_up: bool,
}

#[derive(Clone, Debug)]
pub struct Chord {
	element: Element,
	cache: Cache,

	duration: Duration,
	small: bool,
	note_type: NoteType,

	notes: Vec<Elem<Note>>,
	grace_notes: Vec<Elem<Chord>>,
	stem: Elem<Stem>,
	//stem_slash: Elem<StemSlash>,
	stem_direction: DirectionV,
	hook: Elem<Hook>,
}

impl Chord {
	pub fn new(score: Score) -> Elem<Self> {
		let ret = Elem::new(Self {
			element: Element::new(score.clone()),
			cache: Cache { stem_up: true },
			duration: Duration::default(),
			small: false,
			note_type: NoteType::Normal,

			notes: vec![],
			grace_notes: vec![],
			stem: Stem::new(score.clone()),
			stem_direction: DirectionV::Up,
			hook: Hook::new(score.clone())
		});
		let self_ref = ret.get_self_ref();
		ret.borrow_mut().stem.set_parent(Some(self_ref.clone()));
		ret.borrow_mut().hook.set_parent(Some(self_ref));
		ret
	}
}

impl ElementTrait for Chord {
	fn el(&self) -> &Element { &self.element }
	fn el_mut(&mut self) -> &mut Element { &mut self.element }
	fn element_type(&self) -> ElementType { ElementType::Chord }
}

impl Chord {
	pub fn stem(&self) -> &Elem<Stem> { &self.stem }
	//pub fn stem_slash(&self) -> &StemSlash { self.expect_neighbor(self.stem_slash) }
	//pub fn hook(&self) -> &Hook { self.expect_neighbor(self.hook) }
	pub fn stem_up(&self) -> bool { self.cache.stem_up }

	pub fn duration(&self) -> &Duration { &self.duration }
	pub fn set_duration(&mut self, duration: Duration) { self.duration = duration } //TODO: update note duraitons
	pub fn stem_direction(&self) -> DirectionV { self.stem_direction }
	pub fn set_stem_direction(&mut self, v: DirectionV) { self.stem_direction = v }

	pub fn is_grace(&self) -> bool { self.note_type != NoteType::Normal }
	pub fn down_note(&self) -> Option<&Elem<Note>> {
		self.notes.iter().min_by(|a, b| Elem::borrow(a).line().cmp(&Elem::borrow(b).line()))
	}
	pub fn up_note(&self) -> Option<&Elem<Note>> {
		self.notes.iter().max_by(|a, b| Elem::borrow(a).line().cmp(&Elem::borrow(b).line()))
	}

	pub fn add_note(&mut self, mut note: Elem<Note>) {
		note.set_parent(Some(self.get_self_ref()));
		self.notes.push(note);
	}

	pub fn notehead_width(&self) -> f32 {
		// TODO: check is grace note
		// TODO: Style graceNoteMag
		self.score().note_head_width() * self.scale()
	}
	pub fn compute_up(&mut self) {
		if self.stem_direction != DirectionV::Auto {
			self.cache.stem_up = self.stem_direction == DirectionV::Up
		} else if self.parent().is_none() {
			self.cache.stem_up = self.up_note().map(|n| n.borrow().line() < 4).unwrap_or(true)
		} else if self.is_grace() { // stem direction for grace notes
			self.cache.stem_up = true // Check voice if uneven but default to true
		}  // TODO: Check voice if uneven but default to true
		else {
			let dn_max_line = 4; // TODO: staff middle line
			let ud = self.up_note().map(|n| n.borrow().line()).unwrap_or(0) - dn_max_line;
			if self.notes.len() == 1 {
				self.cache.stem_up = ud > 0
			} else {
				let dd = self.down_note().map(|n| n.borrow().line()).unwrap_or(0) - dn_max_line;
				if -ud == dd {
					let up: i32 = self.notes.iter().map(|n| {
						if n.borrow().line() <= dn_max_line { -1 } else { 1 }
					}).sum();
					self.cache.stem_up = up > 0
				} else {
					self.cache.stem_up = dd > -ud
				}
			}
		}
	}

	pub fn min_abs_stem_len(&self) -> f32 {
		0.0 // TODO: tremolo
	}
	pub fn default_stem_len(&self) -> f32 {
		if let (Some(un), Some(dn)) = (self.up_note(), self.down_note()) {
			let ul = un.borrow().line();
			let dl = dn.borrow().line();

			let hook_type = self.duration().hook_type();
			let hook_idx = hook_type.index();

			let mut shorten_stem = self.score().style().value_bool(StyleId::ShortenStem as SId);
			if hook_idx >= 2 /*|| tremolo*/ { shorten_stem = false }

			let progression: Spatium = self.score().style().value_spatium(StyleId::ShortStemProgression as SId);
			let mut shortest: Spatium = self.score().style().value_spatium(StyleId::ShortestStem as SId);
			if hook_type != HookType::None {
				if self.stem_up() { shortest = shortest.max(3.); }
				else { shortest = shortest.max(3.5); }
			}

			let mut normal_stem_len = if self.small { 2.5 } else { 3.5 };
			normal_stem_len += hook_adjustment(&self.score().font().name, hook_idx, self.stem_up(), self.small);
			if hook_type != HookType::None && self.stem_up() && self.duration.dots() != 0 {
				// Avoid collision of dot with the hook
				if (ul & 1) != 0 { normal_stem_len += 0.5; }
				else { shorten_stem = false; }
			}

			let line_distance = 1.0; // TODO: consult staff
			let mut stem_len;
			if self.is_grace() {
				// grace notes stems are not subject to normal stem rules
				stem_len = (ul - dl) as f32 * 0.5;
				stem_len *= normal_stem_len * self.score().style().value_f32(StyleId::GraceNoteMag as SId);
				if self.stem_up() { stem_len *= -1. }
			} else {
				// TODO: get staff
				let staff_height = 4. * line_distance;
				if self.stem_up() {  // stem up
					let dy = dl as f32 * 0.5;                        // note-side vert. pos.
					let mut sel = ul as f32 * 0.5 - normal_stem_len; // stem end vert. pos

					// if stem ends above top line (with some exceptions), shorten it
					if shorten_stem && sel < 0.0 && (hook_type == HookType::None || !dn.borrow().mirror()) {
						sel -= sel * progression;
					}
					sel = sel.min(staff_height * 0.5); // if stem ends below ('>') staff mid position, stretch it to mid position
					stem_len = sel - dy;  // actual stem length
					if -stem_len < shortest { stem_len = -shortest } // is stem too short lengthen it to shortest possible length
				} else {  // stem down
					let uy = ul as f32 * 0.5;                        // note-side vert. pos.
					let mut sel = dl as f32 * 0.5 + normal_stem_len; // stem end vert. pos.

					// if stem ends below bottom line (with some exceptions), shorten it
					if shorten_stem && sel > staff_height && (hook_type == HookType::None || dn.borrow().mirror()) {
						sel -= (sel - staff_height) * progression;
					}
					sel = sel.max(staff_height * 0.5); // if stem ends above ('<') staff mid position, stretch it to mid position
					stem_len = sel - uy;  // actual stem length
					if stem_len < shortest { stem_len = shortest } // lengthen it to shortest possible position
				}
			}

			// TODO: adjust for tremolo

			let sign = if self.stem_up() { -1.0 } else { 1.0 };
			let spat = self.spatium();
			let mut stem_len_points = stem_len * self.spatium();
			let min_abs_len = self.min_abs_stem_len();
			if sign * stem_len_points < min_abs_len { stem_len_points = sign * min_abs_len }

			-stem_len_points
		} else {
			0.
		}
	}
	pub fn stem_x(&self) -> f32 { if self.stem_up() { self.notehead_width() } else { 0.0 }}
}

impl Drawable for Chord {
	fn layout(&mut self) {
		if self.notes.is_empty() { return; }
		self.compute_up();

		// TODO: grace notes

		let mut chord_x = self.ipos().x;

		let (mut lb, mut rb, mut lhead) = (0f32, 0f32, 0f32);
		for note in &self.notes {
			note.borrow_mut().layout();
			let x1 = note.pos().x + chord_x;
			let x2 = x1 + note.borrow().head_width();
			lb = lb.max(-x1);
			rb = lb.max(x2);
			lhead = lhead.max(-x1);

			// TODO: accidentals
			// TODO: ties / slurs
		}

		// TODO: layout stem
		if self.duration.has_stem() {
			self.stem().borrow_mut().set_len(self.default_stem_len());
			self.stem().borrow_mut().layout();
			let stem_offset = self.stem().borrow().line_width() * self.scale() * if self.stem_up() { -1.0 } else { 1. };
			let stem_pos = self.stem().borrow().pos();
			self.stem().borrow_mut().set_pos(Point2F::new(self.stem_x() + stem_offset, stem_pos.y));
		}

		// TODO: ledger lines
		// TODO: arpeggio
		// TODO: glissando
		// TODO: dots
		{ // TODO: If using hook instead of beam
			let hook_type = if self.stem_up() { self.duration.hook_type().up() } else { self.duration.hook_type().down() };
			self.hook.borrow_mut().set_hook_type(hook_type);
			self.hook.layout();
			let mut p = self.stem.borrow().hook_pos();
			if self.stem_up() {
				p.y += 0.;//self.hook.bbox().origin.y + self.hook.bbox().size.height; TODO: seems to be unnecessary
				p.x -= self.stem.width();
			} else {
				p.y += /*-self.hook.bbox().origin.y +*/ self.hook.bbox().size.height;
				p.x -= self.stem.width();
			}
			self.hook.set_pos(p);

		}
		// TODO: positon grace note
		// TODO: chordline
	}

	fn draw(&self, painter: PainterRef) {
		painter.translate(self.pos().to_vector());

		for note in &self.notes { note.borrow().draw(painter); }
		self.stem().draw(painter);
		self.hook.draw(painter);

		painter.translate(-self.pos().to_vector());
	}
}

fn hook_adjustment(font: &str, hooks: i32, up: bool, small: bool) -> f32 {
	let fallback = hooks > 5; // && use fallback font

	let font = "Gonville";

	match (font, fallback) {
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
	}
}
