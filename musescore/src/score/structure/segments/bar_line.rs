use crate::score::*;

#[derive(Debug, Clone)]
pub struct Barline {
	span_staff: i32,
	span_from: i32,
	span_to: i32,
	bar_line_type: BarLineType,

	y1: f32,
	y2: f32,

	elements: Vec<ElementRef>,
}

impl SegmentTrait for Barline {}

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