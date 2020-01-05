use crate::score::*;
use crate::font::SymName;

#[derive(Debug, Clone)]
pub struct Notehead {

}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum NoteheadType {
	Whole = 0,
	Half = 1,
	Quarter = 2,
	Brevis = 3,
	Auto = 4,
}

impl NoteheadType {
	pub fn get_symid(&self, dir: DirectionV, group: NoteheadGroup) -> SymName {
		self.get_keyed_symid(dir, group, NoteheadScheme::Normal, Tpc::TpcInvalid, Key::C)
	}

	#[allow(unused_variables)]
	pub fn get_keyed_symid(&self, dir: DirectionV, group: NoteheadGroup, scheme: NoteheadScheme, tpc: Tpc, key: Key) -> SymName {
		(match scheme {
			NoteheadScheme::Normal => NOTE_HEADS[dir as usize][group as usize][*self as usize],
			NoteheadScheme::Pitchname | NoteheadScheme::PitchnameGerman => unimplemented!(),
			NoteheadScheme::ShapeNote4 => unimplemented!(),
			NoteheadScheme::ShapeNote7Aikin | NoteheadScheme::ShapeNote7Funk | NoteheadScheme::ShapeNote7Walker => unimplemented!(),
			NoteheadScheme::Solfege => unimplemented!(),
			NoteheadScheme::SolfegeFixed => unimplemented!(),
		})
	}
}


#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum NoteheadScheme {
	Normal = 0,
	Pitchname = 1,
	PitchnameGerman = 2,
	Solfege = 3,
	SolfegeFixed = 4,
	ShapeNote4 = 5,
	ShapeNote7Aikin = 6,
	ShapeNote7Funk = 7,
	ShapeNote7Walker = 8,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum NoteheadGroup {
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
	Invalid = 61,
}

pub const NOTE_HEAD_GROUP_COUNT: usize = 60;
pub const NOTE_HEAD_COUNT: usize = 4;

const NOTE_HEADS: [[[SymName; NOTE_HEAD_COUNT]; NOTE_HEAD_GROUP_COUNT]; 2] = {[
	{[     // down stem
		[SymName::NoteheadWhole, SymName::NoteheadHalf, SymName::NoteheadBlack, SymName::NoteheadDoubleWhole],
		[SymName::NoteheadXWhole, SymName::NoteheadXHalf, SymName::NoteheadXBlack, SymName::NoteheadXDoubleWhole],
		[SymName::NoteheadPlusWhole, SymName::NoteheadPlusHalf, SymName::NoteheadPlusBlack, SymName::NoteheadPlusDoubleWhole],
		[SymName::NoteheadCircleXWhole, SymName::NoteheadCircleXHalf, SymName::NoteheadCircleX, SymName::NoteheadCircleXDoubleWhole],
		[SymName::NoteheadWholeWithX, SymName::NoteheadHalfWithX, SymName::NoteheadVoidWithX, SymName::NoteheadDoubleWholeWithX],
		[SymName::NoteheadTriangleUpWhole, SymName::NoteheadTriangleUpHalf, SymName::NoteheadTriangleUpBlack, SymName::NoteheadTriangleUpDoubleWhole],
		[SymName::NoteheadTriangleDownWhole, SymName::NoteheadTriangleDownHalf, SymName::NoteheadTriangleDownBlack, SymName::NoteheadTriangleDownDoubleWhole],
		[SymName::NoteheadSlashedWhole1, SymName::NoteheadSlashedHalf1, SymName::NoteheadSlashedBlack1, SymName::NoteheadSlashedDoubleWhole1],
		[SymName::NoteheadSlashedWhole2, SymName::NoteheadSlashedHalf2, SymName::NoteheadSlashedBlack2, SymName::NoteheadSlashedDoubleWhole2],
		[SymName::NoteheadDiamondWhole, SymName::NoteheadDiamondHalf, SymName::NoteheadDiamondBlack, SymName::NoteheadDiamondDoubleWhole],
		[SymName::NoteheadDiamondWholeOld, SymName::NoteheadDiamondHalfOld, SymName::NoteheadDiamondBlackOld, SymName::NoteheadDiamondDoubleWholeOld],
		[SymName::NoteheadCircledWhole, SymName::NoteheadCircledHalf, SymName::NoteheadCircledBlack, SymName::NoteheadCircledDoubleWhole],
		[SymName::NoteheadCircledWholeLarge, SymName::NoteheadCircledHalfLarge, SymName::NoteheadCircledBlackLarge, SymName::NoteheadCircledDoubleWholeLarge],
		[SymName::NoteheadLargeArrowUpWhole, SymName::NoteheadLargeArrowUpHalf, SymName::NoteheadLargeArrowUpBlack, SymName::NoteheadLargeArrowUpDoubleWhole],
		[SymName::NoteheadWhole, SymName::NoteheadHalf, SymName::NoteheadBlack, SymName::NoteheadDoubleWholeSquare],
		[SymName::NoteheadSlashWhiteWhole, SymName::NoteheadSlashWhiteHalf, SymName::NoteheadSlashHorizontalEnds, SymName::NoteheadSlashWhiteWhole],
		[SymName::NoteShapeRoundWhite, SymName::NoteShapeRoundWhite, SymName::NoteShapeRoundBlack, SymName::NoteShapeRoundDoubleWhole],
		[SymName::NoteShapeSquareWhite, SymName::NoteShapeSquareWhite, SymName::NoteShapeSquareBlack, SymName::NoteShapeSquareDoubleWhole],
		[SymName::NoteShapeTriangleRightWhite, SymName::NoteShapeTriangleRightWhite, SymName::NoteShapeTriangleRightBlack, SymName::NoteShapeTriangleRightDoubleWhole],
		[SymName::NoteShapeDiamondWhite, SymName::NoteShapeDiamondWhite, SymName::NoteShapeDiamondBlack, SymName::NoteShapeDiamondDoubleWhole],
		[SymName::NoteShapeTriangleUpWhite, SymName::NoteShapeTriangleUpWhite, SymName::NoteShapeTriangleUpBlack, SymName::NoteShapeTriangleUpDoubleWhole],
		[SymName::NoteShapeMoonWhite, SymName::NoteShapeMoonWhite, SymName::NoteShapeMoonBlack, SymName::NoteShapeMoonDoubleWhole],
		[SymName::NoteShapeTriangleRoundWhite, SymName::NoteShapeTriangleRoundWhite, SymName::NoteShapeTriangleRoundBlack, SymName::NoteShapeTriangleRoundDoubleWhole],
		[SymName::NoteShapeKeystoneWhite, SymName::NoteShapeKeystoneWhite, SymName::NoteShapeKeystoneBlack, SymName::NoteShapeKeystoneDoubleWhole],
		[SymName::NoteShapeQuarterMoonWhite, SymName::NoteShapeQuarterMoonWhite, SymName::NoteShapeQuarterMoonBlack, SymName::NoteShapeQuarterMoonDoubleWhole],
		[SymName::NoteShapeIsoscelesTriangleWhite, SymName::NoteShapeIsoscelesTriangleWhite, SymName::NoteShapeIsoscelesTriangleBlack, SymName::NoteShapeIsoscelesTriangleDoubleWhole],
		[SymName::NoteShapeMoonLeftWhite, SymName::NoteShapeMoonLeftWhite, SymName::NoteShapeMoonLeftBlack, SymName::NoteShapeMoonLeftDoubleWhole],
		[SymName::NoteShapeArrowheadLeftWhite, SymName::NoteShapeArrowheadLeftWhite, SymName::NoteShapeArrowheadLeftBlack, SymName::NoteShapeArrowheadLeftDoubleWhole],
		[SymName::NoteShapeTriangleRoundLeftWhite, SymName::NoteShapeTriangleRoundLeftWhite, SymName::NoteShapeTriangleRoundLeftBlack, SymName::NoteShapeTriangleRoundLeftDoubleWhole],
		[SymName::NoteDoWhole, SymName::NoteDoHalf, SymName::NoteDoBlack, SymName::NoSym],
		[SymName::NoteReWhole, SymName::NoteReHalf, SymName::NoteReBlack, SymName::NoSym],
		[SymName::NoteMiWhole, SymName::NoteMiHalf, SymName::NoteMiBlack, SymName::NoSym],
		[SymName::NoteFaWhole, SymName::NoteFaHalf, SymName::NoteFaBlack, SymName::NoSym],
		[SymName::NoteSoWhole, SymName::NoteSoHalf, SymName::NoteSoBlack, SymName::NoSym],
		[SymName::NoteLaWhole, SymName::NoteLaHalf, SymName::NoteLaBlack, SymName::NoSym],
		[SymName::NoteTiWhole, SymName::NoteTiHalf, SymName::NoteTiBlack, SymName::NoSym],
		[SymName::NoteSiWhole, SymName::NoteSiHalf, SymName::NoteSiBlack, SymName::NoSym],
		[SymName::NoteASharpWhole, SymName::NoteASharpHalf, SymName::NoteASharpBlack, SymName::NoSym],
		[SymName::NoteAWhole, SymName::NoteAHalf, SymName::NoteABlack, SymName::NoSym],
		[SymName::NoteAFlatWhole, SymName::NoteAFlatHalf, SymName::NoteAFlatBlack, SymName::NoSym],
		[SymName::NoteBSharpWhole, SymName::NoteBSharpHalf, SymName::NoteBSharpBlack, SymName::NoSym],
		[SymName::NoteBWhole, SymName::NoteBHalf, SymName::NoteBBlack, SymName::NoSym],
		[SymName::NoteBFlatWhole, SymName::NoteBFlatHalf, SymName::NoteBFlatBlack, SymName::NoSym],
		[SymName::NoteCSharpWhole, SymName::NoteCSharpHalf, SymName::NoteCSharpBlack, SymName::NoSym],
		[SymName::NoteCWhole, SymName::NoteCHalf, SymName::NoteCBlack, SymName::NoSym],
		[SymName::NoteCFlatWhole, SymName::NoteCFlatHalf, SymName::NoteCFlatBlack, SymName::NoSym],
		[SymName::NoteDSharpWhole, SymName::NoteDSharpHalf, SymName::NoteDSharpBlack, SymName::NoSym],
		[SymName::NoteDWhole, SymName::NoteDHalf, SymName::NoteDBlack, SymName::NoSym],
		[SymName::NoteDFlatWhole, SymName::NoteDFlatHalf, SymName::NoteDFlatBlack, SymName::NoSym],
		[SymName::NoteESharpWhole, SymName::NoteESharpHalf, SymName::NoteESharpBlack, SymName::NoSym],
		[SymName::NoteEWhole, SymName::NoteEHalf, SymName::NoteEBlack, SymName::NoSym],
		[SymName::NoteEFlatWhole, SymName::NoteEFlatHalf, SymName::NoteEFlatBlack, SymName::NoSym],
		[SymName::NoteFSharpWhole, SymName::NoteFSharpHalf, SymName::NoteFSharpBlack, SymName::NoSym],
		[SymName::NoteFWhole, SymName::NoteFHalf, SymName::NoteFBlack, SymName::NoSym],
		[SymName::NoteFFlatWhole, SymName::NoteFFlatHalf, SymName::NoteFFlatBlack, SymName::NoSym],
		[SymName::NoteGSharpWhole, SymName::NoteGSharpHalf, SymName::NoteGSharpBlack, SymName::NoSym],
		[SymName::NoteGWhole, SymName::NoteGHalf, SymName::NoteGBlack, SymName::NoSym],
		[SymName::NoteGFlatWhole, SymName::NoteGFlatHalf, SymName::NoteGFlatBlack, SymName::NoSym],
		[SymName::NoteHWhole, SymName::NoteHHalf, SymName::NoteHBlack, SymName::NoSym],
		[SymName::NoteHSharpWhole, SymName::NoteHSharpHalf, SymName::NoteHSharpBlack, SymName::NoSym]
	]},
	{[     // up stem
		[SymName::NoteheadWhole, SymName::NoteheadHalf, SymName::NoteheadBlack, SymName::NoteheadDoubleWhole],
		[SymName::NoteheadXWhole, SymName::NoteheadXHalf, SymName::NoteheadXBlack, SymName::NoteheadXDoubleWhole],
		[SymName::NoteheadPlusWhole, SymName::NoteheadPlusHalf, SymName::NoteheadPlusBlack, SymName::NoteheadPlusDoubleWhole],
		[SymName::NoteheadCircleXWhole, SymName::NoteheadCircleXHalf, SymName::NoteheadCircleX, SymName::NoteheadCircleXDoubleWhole],
		[SymName::NoteheadWholeWithX, SymName::NoteheadHalfWithX, SymName::NoteheadVoidWithX, SymName::NoteheadDoubleWholeWithX],
		[SymName::NoteheadTriangleUpWhole, SymName::NoteheadTriangleUpHalf, SymName::NoteheadTriangleUpBlack, SymName::NoteheadTriangleUpDoubleWhole],
		[SymName::NoteheadTriangleDownWhole, SymName::NoteheadTriangleDownHalf, SymName::NoteheadTriangleDownBlack, SymName::NoteheadTriangleDownDoubleWhole],
		[SymName::NoteheadSlashedWhole1, SymName::NoteheadSlashedHalf1, SymName::NoteheadSlashedBlack1, SymName::NoteheadSlashedDoubleWhole1],
		[SymName::NoteheadSlashedWhole2, SymName::NoteheadSlashedHalf2, SymName::NoteheadSlashedBlack2, SymName::NoteheadSlashedDoubleWhole2],
		[SymName::NoteheadDiamondWhole, SymName::NoteheadDiamondHalf, SymName::NoteheadDiamondBlack, SymName::NoteheadDiamondDoubleWhole],
		[SymName::NoteheadDiamondWholeOld, SymName::NoteheadDiamondHalfOld, SymName::NoteheadDiamondBlackOld, SymName::NoteheadDiamondDoubleWholeOld],
		[SymName::NoteheadCircledWhole, SymName::NoteheadCircledHalf, SymName::NoteheadCircledBlack, SymName::NoteheadCircledDoubleWhole],
		[SymName::NoteheadCircledWholeLarge, SymName::NoteheadCircledHalfLarge, SymName::NoteheadCircledBlackLarge, SymName::NoteheadCircledDoubleWholeLarge],
		// different from down, find source?
		[SymName::NoteheadLargeArrowDownWhole, SymName::NoteheadLargeArrowDownHalf, SymName::NoteheadLargeArrowDownBlack, SymName::NoteheadLargeArrowDownDoubleWhole],
		[SymName::NoteheadWhole, SymName::NoteheadHalf, SymName::NoteheadBlack, SymName::NoteheadDoubleWholeSquare],
		[SymName::NoteheadSlashWhiteWhole, SymName::NoteheadSlashWhiteHalf, SymName::NoteheadSlashHorizontalEnds, SymName::NoteheadSlashWhiteDoubleWhole],
		[SymName::NoteShapeRoundWhite, SymName::NoteShapeRoundWhite, SymName::NoteShapeRoundBlack, SymName::NoteShapeRoundDoubleWhole],
		[SymName::NoteShapeSquareWhite, SymName::NoteShapeSquareWhite, SymName::NoteShapeSquareBlack, SymName::NoteShapeSquareDoubleWhole],
		// different from down
		[SymName::NoteShapeTriangleLeftWhite, SymName::NoteShapeTriangleLeftWhite, SymName::NoteShapeTriangleLeftBlack, SymName::NoteShapeTriangleLeftDoubleWhole],
		[SymName::NoteShapeDiamondWhite, SymName::NoteShapeDiamondWhite, SymName::NoteShapeDiamondBlack, SymName::NoteShapeDiamondDoubleWhole],
		[SymName::NoteShapeTriangleUpWhite, SymName::NoteShapeTriangleUpWhite, SymName::NoteShapeTriangleUpBlack, SymName::NoteShapeTriangleUpDoubleWhole],
		[SymName::NoteShapeMoonWhite, SymName::NoteShapeMoonWhite, SymName::NoteShapeMoonBlack, SymName::NoteShapeMoonDoubleWhole],
		[SymName::NoteShapeTriangleRoundWhite, SymName::NoteShapeTriangleRoundWhite, SymName::NoteShapeTriangleRoundBlack, SymName::NoteShapeTriangleRoundDoubleWhole],
		[SymName::NoteShapeKeystoneWhite, SymName::NoteShapeKeystoneWhite, SymName::NoteShapeKeystoneBlack, SymName::NoteShapeKeystoneDoubleWhole],
		[SymName::NoteShapeQuarterMoonWhite, SymName::NoteShapeQuarterMoonWhite, SymName::NoteShapeQuarterMoonBlack, SymName::NoteShapeQuarterMoonDoubleWhole],
		[SymName::NoteShapeIsoscelesTriangleWhite, SymName::NoteShapeIsoscelesTriangleWhite, SymName::NoteShapeIsoscelesTriangleBlack, SymName::NoteShapeIsoscelesTriangleDoubleWhole],
		[SymName::NoteShapeMoonLeftWhite, SymName::NoteShapeMoonLeftWhite, SymName::NoteShapeMoonLeftBlack, SymName::NoteShapeMoonLeftDoubleWhole],
		[SymName::NoteShapeArrowheadLeftWhite, SymName::NoteShapeArrowheadLeftWhite, SymName::NoteShapeArrowheadLeftBlack, SymName::NoteShapeArrowheadLeftDoubleWhole],
		[SymName::NoteShapeTriangleRoundLeftWhite, SymName::NoteShapeTriangleRoundLeftWhite, SymName::NoteShapeTriangleRoundLeftBlack, SymName::NoteShapeTriangleRoundLeftDoubleWhole],
		[SymName::NoteDoWhole, SymName::NoteDoHalf, SymName::NoteDoBlack, SymName::NoSym],
		[SymName::NoteReWhole, SymName::NoteReHalf, SymName::NoteReBlack, SymName::NoSym],
		[SymName::NoteMiWhole, SymName::NoteMiHalf, SymName::NoteMiBlack, SymName::NoSym],
		[SymName::NoteFaWhole, SymName::NoteFaHalf, SymName::NoteFaBlack, SymName::NoSym],
		[SymName::NoteSoWhole, SymName::NoteSoHalf, SymName::NoteSoBlack, SymName::NoSym],
		[SymName::NoteLaWhole, SymName::NoteLaHalf, SymName::NoteLaBlack, SymName::NoSym],
		[SymName::NoteTiWhole, SymName::NoteTiHalf, SymName::NoteTiBlack, SymName::NoSym],
		[SymName::NoteSiWhole, SymName::NoteSiHalf, SymName::NoteSiBlack, SymName::NoSym],
		[SymName::NoteASharpWhole, SymName::NoteASharpHalf, SymName::NoteASharpBlack, SymName::NoSym],
		[SymName::NoteAWhole, SymName::NoteAHalf, SymName::NoteABlack, SymName::NoSym],
		[SymName::NoteAFlatWhole, SymName::NoteAFlatHalf, SymName::NoteAFlatBlack, SymName::NoSym],
		[SymName::NoteBSharpWhole, SymName::NoteBSharpHalf, SymName::NoteBSharpBlack, SymName::NoSym],
		[SymName::NoteBWhole, SymName::NoteBHalf, SymName::NoteBBlack, SymName::NoSym],
		[SymName::NoteBFlatWhole, SymName::NoteBFlatHalf, SymName::NoteBFlatBlack, SymName::NoSym],
		[SymName::NoteCSharpWhole, SymName::NoteCSharpHalf, SymName::NoteCSharpBlack, SymName::NoSym],
		[SymName::NoteCWhole, SymName::NoteCHalf, SymName::NoteCBlack, SymName::NoSym],
		[SymName::NoteCFlatWhole, SymName::NoteCFlatHalf, SymName::NoteCFlatBlack, SymName::NoSym],
		[SymName::NoteDSharpWhole, SymName::NoteDSharpHalf, SymName::NoteDSharpBlack, SymName::NoSym],
		[SymName::NoteDWhole, SymName::NoteDHalf, SymName::NoteDBlack, SymName::NoSym],
		[SymName::NoteDFlatWhole, SymName::NoteDFlatHalf, SymName::NoteDFlatBlack, SymName::NoSym],
		[SymName::NoteESharpWhole, SymName::NoteESharpHalf, SymName::NoteESharpBlack, SymName::NoSym],
		[SymName::NoteEWhole, SymName::NoteEHalf, SymName::NoteEBlack, SymName::NoSym],
		[SymName::NoteEFlatWhole, SymName::NoteEFlatHalf, SymName::NoteEFlatBlack, SymName::NoSym],
		[SymName::NoteFSharpWhole, SymName::NoteFSharpHalf, SymName::NoteFSharpBlack, SymName::NoSym],
		[SymName::NoteFWhole, SymName::NoteFHalf, SymName::NoteFBlack, SymName::NoSym],
		[SymName::NoteFFlatWhole, SymName::NoteFFlatHalf, SymName::NoteFFlatBlack, SymName::NoSym],
		[SymName::NoteGSharpWhole, SymName::NoteGSharpHalf, SymName::NoteGSharpBlack, SymName::NoSym],
		[SymName::NoteGWhole, SymName::NoteGHalf, SymName::NoteGBlack, SymName::NoSym],
		[SymName::NoteGFlatWhole, SymName::NoteGFlatHalf, SymName::NoteGFlatBlack, SymName::NoSym],
		[SymName::NoteHWhole, SymName::NoteHHalf, SymName::NoteHBlack, SymName::NoSym],
		[SymName::NoteHSharpWhole, SymName::NoteHSharpHalf, SymName::NoteHSharpBlack, SymName::NoSym]
	]}
]};