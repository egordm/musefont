use crate::font::SymName;
use crate::score::*;

/// # Accidental
#[derive(Debug, Clone)]
pub struct Accidental {
	element: ElementData,

	elements: Vec<SymElement>,
	accidental_type: AccidentalType,
	small: bool,
	bracket: AccidentalBracket,
	role: AccidentalRole,
}

impl Element for Accidental {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Accidental }
}

#[derive(Debug, Clone)]
pub struct SymElement {
	sym: SymName,
	x: f32,
}

impl SymElement {
	pub fn new(sym: SymName, x: f32) -> Self {Self{sym, x}}
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum AccidentalBracket {
	None = 0,
	Parenthesis = 1,
	Bracket = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AccidentalRole {
	/// layout created accidental
	Auto,
	/// user created accidental
	User
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum AccidentalType {
	None = 0,
	Flat = 1,
	Natural = 2,
	Sharp = 3,
	Sharp2 = 4,
	Flat2 = 5,
	//SHARP3
    //FLAT3
	NaturalFlat = 6,
	NaturalSharp = 7,
	SharpSharp = 8,

	// Gould arrow quartertone
	FlatArrowUp = 9,
	FlatArrowDown = 10,
	NaturalArrowUp = 11,
	NaturalArrowDown = 12,
	SharpArrowUp = 13,
	SharpArrowDown = 14,
	Sharp2ArrowUp = 15,
	Sharp2ArrowDown = 16,
	Flat2ArrowUp = 17,
	Flat2ArrowDown = 18,

	// Stein-Zimmermann
	MirroredFlat = 19,
	MirroredFlat2 = 20,
	SharpSlash = 21,
	SharpSlash4 = 22,

	// Arel-Ezgi-Uzdilek (AEU)
	FlatSlash2 = 23,
	FlatSlash = 24,
	SharpSlash3 = 25,
	SharpSlash2 = 26,

	// Extended Helmholtz-Ellis accidentals (just intonation)
	DoubleFlatOneArrowDown = 27,
	FlatOneArrowDown = 28,
	NaturalOneArrowDown = 29,
	SharpOneArrowDown = 30,
	DoubleSharpOneArrowDown = 31,
	DoubleFlatOneArrowUp = 32,

	FlatOneArrowUp = 33,
	NaturalOneArrowUp = 34,
	SharpOneArrowUp = 35,
	DoubleSharpOneArrowUp = 36,
	DoubleFlatTwoArrowsDown = 37,
	FlatTwoArrowsDown = 38,

	NaturalTwoArrowsDown = 39,
	SharpTwoArrowsDown = 40,
	DoubleSharpTwoArrowsDown = 41,
	DoubleFlatTwoArrowsUp = 42,
	FlatTwoArrowsUp = 43,
	NaturalTwoArrowsUp = 44,

	SharpTwoArrowsUp = 45,
	DoubleSharpTwoArrowsUp = 46,
	DoubleFlatThreeArrowsDown = 47,
	FlatThreeArrowsDown = 48,
	NaturalThreeArrowsDown = 49,
	SharpThreeArrowsDown = 50,

	DoubleSharpThreeArrowsDown = 51,
	DoubleFlatThreeArrowsUp = 52,
	FlatThreeArrowsUp = 53,
	NaturalThreeArrowsUp = 54,
	SharpThreeArrowsUp = 55,
	DoubleSharpThreeArrowsUp = 56,

	LowerOneSeptimalComma = 57,
	RaiseOneSeptimalComma = 58,
	LowerTwoSeptimalCommas = 59,
	RaiseTwoSeptimalCommas = 60,
	LowerOneUndecimalQuartertone = 61,
	RaiseOneUndecimalQuartertone = 62,

	LowerOneTridecimalQuartertone = 63,
	RaiseOneTridecimalQuartertone = 64,

	DoubleFlatEqualTempered = 65,
	FlatEqualTempered = 66,
	NaturalEqualTempered = 67,
	SharpEqualTempered = 68,
	DoubleSharpEqualTempered = 69,
	QuarterFlatEqualTempered = 70,
	QuarterSharpEqualTempered = 71,

	// Persian
	Sori = 72,
	Koron = 73,
	End = 74,
}
