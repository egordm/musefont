use crate::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value {
	line: i32,
	fret: i32,
	string: i32,
}

impl Default for Value {
	fn default() -> Self {
		Self {
			line: 0,
			fret: 0,
			string: 0,
		}
	}
}

#[derive(Clone)]
pub struct Note {
	element: Element,
	head_group: HeadGroup,
	head_type: HeadType,
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
			head_group: HeadGroup::Normal,
			head_type: HeadType::Auto,
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

impl ElementTrait for Note {
	fn el(&self) -> &Element { &self.element }

	fn el_mut(&mut self) -> &mut Element { &mut self.element }
}


#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum HeadGroup {
	Normal = 0,
	Cross = 1,
	Plus = 2,
	XCircle = 3,
	Withx = 4,
	TriangleUp = 5,
	TriangleDown = 6,
	Slashed1 = 7,
	Slashed2 = 8,
	Diamond = 9,
	DiamondOld = 10,
	Circled = 11,
	CircledLarge = 12,
	LargeArrow = 13,
	BrevisAlt = 14,

	Slash = 15,

	Sol = 16,
	La = 17,
	Fa = 18,
	Mi = 19,
	Do = 20,
	Re = 21,
	Ti = 22,

	DoWalker = 23,
	ReWalker = 24,
	TiWalker = 25,
	DoFunk = 26,
	ReFunk = 27,
	TiFunk = 28,

	DoName = 29,
	ReName = 30,
	MiName = 31,
	FaName = 32,
	SolName = 33,
	LaName = 34,
	TiName = 35,
	SiName = 36,

	ASharp = 37,
	A = 38,
	AFlat = 39,
	BSharp = 40,
	B = 41,
	BFlat = 42,
	CSharp = 43,
	C = 44,
	CFlat = 45,
	DSharp = 46,
	D = 47,
	DFlat = 48,
	ESharp = 49,
	E = 50,
	EFlat = 51,
	FSharp = 52,
	F = 53,
	FFlat = 54,
	GSharp = 55,
	G = 56,
	GFlat = 57,
	H = 58,
	HSharp = 59,

	Custom = 60,
	Groups = 61,
	Invalid = 62,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum HeadType {
	Auto = 0,
	Whole = 1,
	Half = 2,
	Quarter = 3,
	Brevis = 4,
	Types = 5,
}