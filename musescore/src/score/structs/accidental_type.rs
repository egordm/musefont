use crate::font::SymName;

pub const ACC_LIST: [(AccidentalVal, i32, SymName); 74] = {[
	(AccidentalVal::Natural, 0, SymName::NoSym),                // NONE
	(AccidentalVal::Flat, 0, SymName::AccidentalFlat),       // FLAT
	(AccidentalVal::Natural, 0, SymName::AccidentalNatural),    // NATURAL
	(AccidentalVal::Sharp, 0, SymName::AccidentalSharp),      // SHARP
	(AccidentalVal::Sharp2, 0, SymName::AccidentalDoubleSharp),// SHARP2
	(AccidentalVal::Flat2, 0, SymName::AccidentalDoubleFlat), // FLAT2
	//(AccidentalVal::SHARP3,  0,    SymName::AccidentalTripleSharp),// SHARP3
	//(AccidentalVal::FLAT3,   0,    SymName::AccidentalTripleFlat), // FLAT3
	(AccidentalVal::Flat, 0, SymName::AccidentalNaturalFlat),  // NATURAL_FLAT
	(AccidentalVal::Sharp, 0, SymName::AccidentalNaturalSharp), // NATURAL_SHARP
	(AccidentalVal::Sharp2, 0, SymName::AccidentalSharpSharp),   // SHARP_SHARP

	// Gould arrow quartertone
	(AccidentalVal::Natural, -50, SymName::AccidentalQuarterToneFlatArrowUp),        // FLAT_ARROW_UP
	(AccidentalVal::Natural, -150, SymName::AccidentalThreeQuarterTonesFlatArrowDown),// FLAT_ARROW_DOWN
	(AccidentalVal::Natural, 50, SymName::AccidentalQuarterToneSharpNaturalArrowUp),// NATURAL_ARROW_UP
	(AccidentalVal::Natural, -50, SymName::AccidentalQuarterToneFlatNaturalArrowDown), // NATURAL_ARROW_DOWN
	(AccidentalVal::Natural, 150, SymName::AccidentalThreeQuarterTonesSharpArrowUp), // SHARP_ARROW_UP
	(AccidentalVal::Natural, 50, SymName::AccidentalQuarterToneSharpArrowDown),     // SHARP_ARROW_DOWN
	(AccidentalVal::Natural, 250, SymName::AccidentalFiveQuarterTonesSharpArrowUp),    // SHARP2_ARROW_UP
	(AccidentalVal::Natural, 150, SymName::AccidentalThreeQuarterTonesSharpArrowDown), // SHARP2_ARROW_DOWN
	(AccidentalVal::Natural, -250, SymName::AccidentalThreeQuarterTonesFlatArrowUp),    // FLAT2_ARROW_UP
	(AccidentalVal::Natural, -150, SymName::AccidentalFiveQuarterTonesFlatArrowDown),   // FLAT2_ARROW_DOWN

	// Stein-Zimmermann
	(AccidentalVal::Natural, -50, SymName::AccidentalQuarterToneFlatStein), // MIRRORED_FLAT
	(AccidentalVal::Natural, -150, SymName::AccidentalThreeQuarterTonesFlatZimmermann), // MIRRORED_FLAT2
	(AccidentalVal::Natural, 50, SymName::AccidentalQuarterToneSharpStein),       // SHARP_SLASH
	(AccidentalVal::Natural, 150, SymName::AccidentalThreeQuarterTonesSharpStein), // SHARP_SLASH4

	//Arel-Ezgi-Uzdilek (AEU)
	(AccidentalVal::Natural, 0, SymName::AccidentalBuyukMucennebFlat),  // FLAT_SLASH2
	(AccidentalVal::Natural, 0, SymName::AccidentalBakiyeFlat),         // FLAT_SLASH
	(AccidentalVal::Natural, 0, SymName::AccidentalKucukMucennebSharp), // SHARP_SLASH3
	(AccidentalVal::Natural, 0, SymName::AccidentalBuyukMucennebSharp), // SHARP_SLASH2

	// Extended Helmholtz-Ellis accidentals (just intonation)
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleFlatOneArrowDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalFlatOneArrowDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalNaturalOneArrowDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalSharpOneArrowDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleSharpOneArrowDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleFlatOneArrowUp),

	(AccidentalVal::Natural, 0, SymName::AccidentalFlatOneArrowUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalNaturalOneArrowUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalSharpOneArrowUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleSharpOneArrowUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleFlatTwoArrowsDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalFlatTwoArrowsDown),

	(AccidentalVal::Natural, 0, SymName::AccidentalNaturalTwoArrowsDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalSharpTwoArrowsDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleSharpTwoArrowsDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleFlatTwoArrowsUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalFlatTwoArrowsUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalNaturalTwoArrowsUp),

	(AccidentalVal::Natural, 0, SymName::AccidentalSharpTwoArrowsUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleSharpTwoArrowsUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleFlatThreeArrowsDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalFlatThreeArrowsDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalNaturalThreeArrowsDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalSharpThreeArrowsDown),

	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleSharpThreeArrowsDown),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleFlatThreeArrowsUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalFlatThreeArrowsUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalNaturalThreeArrowsUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalSharpThreeArrowsUp),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleSharpThreeArrowsUp),

	(AccidentalVal::Natural, 0, SymName::AccidentalLowerOneSeptimalComma),
	(AccidentalVal::Natural, 0, SymName::AccidentalRaiseOneSeptimalComma),
	(AccidentalVal::Natural, 0, SymName::AccidentalLowerTwoSeptimalCommas),
	(AccidentalVal::Natural, 0, SymName::AccidentalRaiseTwoSeptimalCommas),
	(AccidentalVal::Natural, 0, SymName::AccidentalLowerOneUndecimalQuartertone),
	(AccidentalVal::Natural, 0, SymName::AccidentalRaiseOneUndecimalQuartertone),

	(AccidentalVal::Natural, 0, SymName::AccidentalLowerOneTridecimalQuartertone),
	(AccidentalVal::Natural, 0, SymName::AccidentalRaiseOneTridecimalQuartertone),

	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleFlatEqualTempered),
	(AccidentalVal::Natural, 0, SymName::AccidentalFlatEqualTempered),
	(AccidentalVal::Natural, 0, SymName::AccidentalNaturalEqualTempered),
	(AccidentalVal::Natural, 0, SymName::AccidentalSharpEqualTempered),
	(AccidentalVal::Natural, 0, SymName::AccidentalDoubleSharpEqualTempered),
	(AccidentalVal::Natural, 0, SymName::AccidentalQuarterFlatEqualTempered),
	(AccidentalVal::Natural, 0, SymName::AccidentalQuarterSharpEqualTempered),

	// Persian
	(AccidentalVal::Natural, 33, SymName::AccidentalSori),                          // SORI
	(AccidentalVal::Natural, -67, SymName::AccidentalKoron),                         // KORON
]};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AccidentalVal {
	Sharp2 = 2,
	Sharp = 1,
	Natural = 0,
	Flat = -1,
	Flat2 = -2,
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
}
