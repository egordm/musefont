bitflags! { pub struct FontStyle: u32 {
	const NORMAL = 0x00000000;
	const BOLD = 0x00000001;
	const ITALIC = 0x00000002;
	const UNDERLINE = 0x00000004;
}}

impl Into<u32> for FontStyle {
	fn into(self) -> u32 { self.bits }
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum FrameType {
	NoFrame = 0,
	Square = 1,
	Circle = 2,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum HookStyleType {
	None = 0,
	Hook90 = 1,
	Hook45 = 2,
	Hook90T = 3,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum LineStyle { // pen style
NoPen = 0,
	SolidLine = 1,
	DashLine = 2,
	DotLine = 3,
	DashDotLine = 4,
	DashDotDotLine = 5,
	CustomDashLine = 6,
}


#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum TremoloPlacement {
	Default = 0,
	StemCenter = 1,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum KeySigNatural {
	None = 0,             // no naturals, except for change to CMaj/Amin
	Before = 1,             // naturals before accidentals
	After = 2,              // naturals after accidentals (but always before if going sharps <=> flats)
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum TupletNumberType {
	ShowNumber = 0,
	ShowRelation = 1,
	NoText = 2,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum TupletBracketType {
	AutoBracket = 0,
	ShowBracket = 1,
	ShowNoBracket = 2,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum OffsetType {
	/// offset in point units
	Abs = 1,
	/// offset in staff space units
	Spatium = 2,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum VerticalAlignRange {
	Segment = 0,
	Measure = 1,
	System = 2,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum OrnamentStyle {
	Default = 0,
	Baroque = 1,
}
