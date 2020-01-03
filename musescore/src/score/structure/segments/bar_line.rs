use crate::score::*;

#[derive(Debug, Clone)]
pub struct Barline {
	element: ElementData,
	segment_data: SegmentData,

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
}

impl SegmentTrait for Barline {
	fn segment_data(&self) -> &SegmentData { &self.segment_data }
	fn segment_data_mut(&mut self) -> &mut SegmentData { &mut self.segment_data }
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