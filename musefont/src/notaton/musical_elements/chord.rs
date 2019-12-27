use crate::*;

#[derive(Clone, Debug)]
pub struct Chord {
	element: Element,
	notes: Vec<Elem<Note>>,
	grace_notes: Vec<Elem<Note>>,
	//stem: Elem<Stem>,
	//stem_slash: Elem<StemSlash>,
	stem_direction: DirectionH,
	//hook: Elem<Hook>,
}

impl ElementTrait for Chord {
	fn el(&self) -> &Element { &self.element }
	fn el_mut(&mut self) -> &mut Element { &mut self.element }
	fn element_type(&self) -> ElementType { ElementType::Chord }
}

impl Chord {
	//pub fn stem(&self) -> &Stem { self.expect_neighbor(self.stem) }
	//pub fn stem_slash(&self) -> &StemSlash { self.expect_neighbor(self.stem_slash) }
	//pub fn hook(&self) -> &Hook { self.expect_neighbor(self.hook) }

	pub fn low_note(&self) -> Option<&Elem<Note>> {
		self.notes.iter().min_by(|a, b| a.borrow().line().cmp(&b.borrow().line()))
	}
	pub fn high_note(&self) -> Option<&Elem<Note>> {
		self.notes.iter().max_by(|a, b| a.borrow().line().cmp(&b.borrow().line()))
	}
}
/*

impl Drawable for Chord {
	fn layout(&mut self, data: &LayoutData) {
		let mut chord_x = self.ipos().x;

		let (mut lb, mut rb) = (0f32, 0f32);
		for note in &mut self.notes {
			note.layout(data);
			let x1 = note.pos().x + chord_x;
			let x2 = x1 + note.head_width(data);

			lb = lb.max(-x1);
			rb = lb.max(x2);

			// TODO: check note ornaments like accidentals and ties
		}

		// TODO: layout line

		// TODO: dots
	}

	fn draw(&self, painter: PainterRef) {
		unimplemented!()
	}
}*/
