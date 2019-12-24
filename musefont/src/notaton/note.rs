use crate::*;

pub const FRET_NONE: i32 = -1;
pub const STRING_NONE: i32 = -1;
pub const INVALID_LINE: i32 = -10000;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value {
	pitch: i32,
	line: i32,
	fret: i32,
	string: i32,
}

impl Default for Value {
	fn default() -> Self {
		Self {
			pitch: -1,
			line: INVALID_LINE,
			fret: FRET_NONE,
			string: STRING_NONE,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Note {
	element: Element,
	duration: Duration,
	head_group: notehead::Group,
	head_type: notehead::Type,
	ghost: bool,

	accidental: Accidental,
	//tie_for: Tie,
	//tie_back: Tie,
	dots: Vec<NoteDot>,

	user_mirror: DirectionH,
	user_dot_pos: DirectionV,

	articulations: u32,
	value: Value,

	cache_notehead_sym: SymId,
}

impl Default for Note {
	fn default() -> Self {
		Self {
			element: Element::default(),
			duration: Duration::default(),
			head_group: notehead::Group::Normal,
			head_type: notehead::Type::Auto,
			ghost: false,
			accidental: Accidental::default(),
			dots: Vec::new(),
			user_mirror: DirectionH::Auto,
			user_dot_pos: DirectionV::Auto,
			articulations: 0,
			value: Value::default(),
			cache_notehead_sym: SymIdent::NoSym.id()
		}
	}
}

impl Note {
	pub fn value(&self) -> &Value { &self.value }

	pub fn note_head(&self) -> SymId {
		// TODO: check if correspond to a chord && override
		let dir = DirectionV::Up;
		let mut head = self.duration.ty().note_head();

		if self.head_type != notehead::Type::Auto { head = self.head_type }

		let key = Key::C;
		let scheme = notehead::Scheme::Normal;
		// TODO: override

		let ret = head.get_keyed_symid(dir, self.head_group, scheme, 0, key);
		if SymIdent::NoSym == ret {
			head.get_keyed_symid(dir, notehead::Group::Normal, scheme, 0, key)
		} else {
			ret
		}
	}
}

impl ElementTrait for Note {
	fn el(&self) -> &Element { &self.element }

	fn el_mut(&mut self) -> &mut Element { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Note }
}