use crate::*;

#[derive(Clone, Debug)]
pub struct Stem {
	element: Element,
	line: LineF,
	line_width: f32,
	len: f32,
}

impl Stem {
	pub fn new(score: Score) -> Elem<Self> { Elem::new(Self {
		element: Element::new(score),
		line: LineF::new(Point2F::default(), Point2F::default()),
		line_width: 0.02,
		len: 0.0
	})}
}

impl ElementTrait for Stem {
	fn el(&self) -> &Element { &self.element }
	fn el_mut(&mut self) -> &mut Element { &mut self.element }
	fn element_type(&self) -> ElementType { ElementType::Note }
}

impl Stem {
	pub fn chord(&self) -> Option<Elem<Chord>> { self.parent_ty::<Chord>() }

	pub fn line_width(&self) -> f32 { self.line_width }
	pub fn set_line_width(&mut self, v: f32) { self.line_width = v }

	pub fn len(&self) -> f32 { self.len }
	pub fn set_len(&mut self, v: f32) { self.len = v; self.layout() }

	pub fn up(&self) -> bool {
		if let Some(chord) = self.chord() {
			chord.borrow().stem_up()
		} else { true }
	}
	pub fn hook_pos(&self) -> Point2F {
		let mut p = self.pos() + self.line.p2.to_vector();
		let xoff = self.line_width() * 0.5 * self.scale();
		p.x += xoff;
		p
	}
	pub fn stem_len(&self) -> f32 {
		if self.up() { -self.len() } else { self.len() }
	}
	pub fn p2(&self) -> Point2F { self.line.p2 }
}

impl Drawable for Stem {
	fn layout(&mut self) {
		let l = self.len;
		let up = self.up();
		let vscale = if up { -1. } else { 1. };

		let mut y1 = 0.;
		if let Some(chord) = self.chord() {
			self.set_scale(chord.scale());

			// move stem start to note attach point
			let chord = chord.borrow();
			if let Some(note) = if up { chord.low_note() } else { chord.high_note() } {
				y1 += if up { note.borrow().stem_up_se().y } else { note.borrow().stem_down_nw().y };
				self.set_pos(Point2F::new(self.x(), note.y()));
			}
		}

		let lw5 = self.line_width() * 0.5 * self.scale();
		self.line = LineF::new(Point2F::new(0., y1), Point2F::new(0., l));
		let r = rect_norm(rect_from_points(self.line.p1, self.line.p2));
		self.set_bbox(rect_adjust(r, Point2F::new(-lw5, -lw5), Point2F::new(lw5, lw5)));
	}

	fn draw(&self, painter: PainterRef) {
		painter.translate(self.pos().to_vector());

		painter.draw(DrawIns::Line(self.line.clone(), self.line_width()));

		painter.translate(-self.pos().to_vector());
	}
}