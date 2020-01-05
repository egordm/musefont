use crate::*;
use crate::score::*;

/// # ChordLine
/// bezier line attached to top note of a chord implements fall, doit, plop, bend
#[derive(Debug, Clone)]
pub struct Chordline {
	element: ElementData,

	chord_line_type: ChordLineType,
	straight: bool,
	path: LineF,
	modified: bool,
	length_x: f32,
	length_y: f32,
	initial_length: i32,
}

impl Chordline {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		chord_line_type: ChordLineType::Notype,
		straight: false,
		path: LineF::default(),
		modified: false,
		length_x: 0.0,
		length_y: 0.0,
		initial_length: 2
	})}

	pub fn chord_line_type(&self) -> ChordLineType { self.chord_line_type }
	pub fn set_chord_line_type(&mut self, v: ChordLineType) { self.chord_line_type = v }

	pub fn straight(&self) -> bool { self.straight }
	pub fn set_straight(&mut self, v: bool) { self.straight = v }

	pub fn length_x(&self) -> f32 { self.length_x }
	pub fn set_length_x(&mut self, v: f32) { self.length_x = v }

	pub fn length_y(&self) -> f32 { self.length_y }
	pub fn set_length_y(&mut self, v: f32) { self.length_y = v }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::ChordLineType => ValueVariant::from_enum(self.chord_line_type()),
			PropertyId::ChordLineStraight => self.straight().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::ChordLineType => v.with_enum(|v| self.set_chord_line_type(v)),
			PropertyId::ChordLineStraight => v.with_value(|v| self.set_straight(v)),
			_ => false,
		}
	}
}

impl Element for Chordline {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Chordline }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl AtomTrait for Chordline {

}


#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum ChordLineType {
	Notype = 0,
	Fall = 1,
	Doit = 2,
	Plop = 3,
	Scoop = 4,
}