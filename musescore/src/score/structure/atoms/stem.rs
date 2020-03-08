use crate::*;
use crate::score::*;

#[derive(Debug, Clone)]
pub struct Stem {
	element: ElementData,

	line: LineF,
	line_width: f32,
	user_len: f32,
	len: f32,
}

impl Stem {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		line: Default::default(),
		line_width: 0.0,
		user_len: 0.0,
		len: 0.0
	})}

	// Returns line width unscaled
	pub fn line_width(&self) -> f32 { self.line_width }
	pub fn set_line_width(&mut self, v: f32) {
		self.line_width = v
	}

	pub fn user_len(&self) -> f32 { self.user_len }
	pub fn set_user_len(&mut self, v: f32) { self.user_len = v }

	pub fn len(&self) -> f32 { self.len }
	pub fn set_len(&mut self, v: f32) { self.len = v.abs() }

	pub fn up(&self) -> bool {
		if let Some(chord) = self.chord() { chord.as_trait().up() } else { true }
	}
	pub fn stem_len(&self) -> f32 { if self.up() { -self.len } else { self.len }}
	pub fn p2(&self) -> Point2F { self.line.p2 }
	/// in chord coordinates
	pub fn hook_pos(&self) -> Point2F {
		let mut p = self.pos() + self.line.p2.to_vector();
		p.x += self.line_width * 0.5 * self.scale(); // TODO: unneeded?
		p
	}

	pub(crate) fn set_line(&mut self, line: LineF) { self.line = line }
	pub fn line(&self) -> &LineF { &self.line }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::LineWidth => self.line_width().into(),
			PropertyId::UserLen => self.user_len().into(),
			PropertyId::StemDirection => {
				if let Some(ChordRef::Chord(chord)) = self.chord() {
					ValueVariant::from_enum(chord.borrow_el().stem_direction())
				} else { ValueVariant::None }

			},
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::LineWidth => v.with_value(|v| self.set_line_width(v)),
			PropertyId::UserLen => v.with_value(|v| self.set_user_len(v)),
			PropertyId::StemDirection => v.with_enum(|v| {
				if let Some(ChordRef::Chord(chord)) = self.chord() {
					chord.borrow_mut_el().set_stem_direction(v)
				}
			}),
			_ => false,
		}
	}
}

impl Element for Stem {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Stem }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p).if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_custom_property(p, v.clone()) || self.set_element_property(p, v)
	}
}

impl AtomTrait for Stem {

}
