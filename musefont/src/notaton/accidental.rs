use crate::*;

#[derive(Clone, Debug)]
pub struct Accidental {
	element: Element,
	accidental_type: Type,
	small: bool,
	bracket: Bracket,
}

impl Default for Accidental {
	fn default() -> Self {
		Self {
			element: Element::default(),
			accidental_type: Type::None,
			small: false,
			bracket: Bracket::None
		}
	}
}

impl ElementTrait for Accidental {
	fn el(&self) -> &Element { &self.element }

	fn el_mut(&mut self) -> &mut Element { &mut self.element }
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Bracket {
	None = 0,
	Parenthesis = 1,
	Bracket = 2,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Type {
	None = 0,
	Flat = 1,
	Natural = 2,
	Sharp = 3,
	Sharp2 = 4,
	Flat2 = 5,
	NaturalFlat = 6,
	NaturalSharp = 7,
	SharpSharp = 8,
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
	MirroredFlat = 19,
	MirroredFlat2 = 20,
	SharpSlash = 21,
	SharpSlash4 = 22,
	FlatSlash2 = 23,
	FlatSlash = 24,
	SharpSlash3 = 25,
	SharpSlash2 = 26,
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
	Sori = 72,
	Koron = 73,
	End = 74,
}