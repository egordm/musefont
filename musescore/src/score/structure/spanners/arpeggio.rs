use crate::font::SymId;
use crate::score::*;

#[derive(Debug, Clone)]
pub struct Arpeggio {
	element: ElementData,

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

impl Element for Arpeggio {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Arpeggio }
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