use crate::font::SymId;

#[derive(Debug, Clone)]
pub struct Arpeggio {
	arpeggio_type: ArpeggioType,
	user_len1: f32,
	user_len2: f32,
	height: f32,
	/// spanning staves
	span: i32,

	symbols: Vec<SymId>,
	stretch: f32,
	/// set in layout, will skip draw if true
	hidden: bool,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum ArpeggioType {
	Normal = 0,
	Up = 1,
	Down = 2,
	Bracket = 3,
	UpStraight = 4,
	DownStraight = 5,
}