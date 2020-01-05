use crate::score::*;

#[derive(Debug, Clone)]
pub struct Barline {
	element: ElementData,

	/// span barline to next staff if true, values > 1 are used for importing from 2.x
	span_staff: i32,
	/// line number on start and end staves
	span_from: i32,
	span_to: i32,
	bar_line_type: BarLineType,

	y1: f32,
	y2: f32,

	/// fermata or other articulations
	elements: Vec<ElementRef>,
}

impl Element for Barline {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Barline }
}

impl SegmentTrait for Barline {
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BarLineType {
	Normal = 1,
	Double = 2,
	StartRepeat = 4,
	EndRepeat = 8,
	Broken = 0x10,
	End = 0x20,
	EndStartRepeat = 0x40,
	Dotted = 0x80
}