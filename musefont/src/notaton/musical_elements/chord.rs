use crate::*;

#[derive(Clone, Debug)]
pub struct Chord {
	element: Element,
	duration: Duration,
	notes: Vec<Elem<Note>>,
	grace_notes: Vec<Elem<Chord>>,
	stem: Elem<Stem>,
	//stem_slash: Elem<StemSlash>,
	stem_direction: DirectionH,
	stem_up: bool,
	//hook: Elem<Hook>,
}

impl Chord {
	pub fn new(score: Score) -> Elem<Self> {
		let ret = Elem::new(Self {
			element: Element::new(score.clone()),
			duration: Duration::default(),
			notes: vec![],
			grace_notes: vec![],
			stem: Stem::new(score.clone()),
			stem_direction: DirectionH::Right,
			stem_up: true
		});
		let self_ref = ret.get_self_ref();
		ret.borrow_mut().stem.set_parent(Some(self_ref));
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
	pub fn stem_up(&self) -> bool { self.stem_up }

	pub fn low_note(&self) -> Option<&Elem<Note>> {
		self.notes.iter().min_by(|a, b| Elem::borrow(a).line().cmp(&Elem::borrow(b).line()))
	}
	pub fn high_note(&self) -> Option<&Elem<Note>> {
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
	pub fn default_stem_len(&self) -> f32 {
		if let (Some(un), Some(dn)) = (self.high_note(), self.low_note()) {
			let shortest = 80.;
			let ul = un.borrow().line() as f32;
			let dn = dn.borrow().line() as f32;
			((ul - dn) * 0.5).max(shortest)
		} else {
			0.
		}
	}
	pub fn stem_x(&self) -> f32 { if self.stem_up() { self.notehead_width() } else { 0.0 }}
}

impl Drawable for Chord {
	fn layout(&mut self) {
		if self.notes.is_empty() { return; }
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
			let stem_offset = self.stem().borrow().line_width() * -1.0 * self.scale();
			let stem_pos = self.stem().borrow().pos();
			self.stem().borrow_mut().set_pos(Point2F::new(self.stem_x() + stem_offset, stem_pos.y));
		}

		// TODO: ledger lines
		// TODO: arpeggio
		// TODO: glissando
		// TODO: dots
		// TODO: hook
		// TODO: positon grace note
		// TODO: chordline
	}

	fn draw(&self, painter: PainterRef) {
		painter.translate(self.pos().to_vector());

		for note in &self.notes { note.borrow().draw(painter); }
		self.stem().borrow().draw(painter);

		painter.translate(-self.pos().to_vector());
	}
}
