use crate::score::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct Slur {
	element: ElementData,

	spanner_data: SpannerData,
	line_type: LineType,
	/// actual direction
	up: bool,
	slur_direction: DirectionV,
}

impl Element for Slur {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}

#[derive(Debug, Clone)]
pub struct SlurSegment {
	element: ElementData,

	segment_data: SpannerSegment,
	ups: UP,
	path: LineType, // TODO: path type
	shape_path: LineType, // TODO: path type
	// TODO: shape
}

impl Element for SlurSegment {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}

#[derive(Debug, Clone)]
pub struct UP {
	/// layout position relative to pos()
	p: Point2F,
	/// user offset in point units
	off: Point2F,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum LineType {
	Solid = 0,
	Dotted = 1,
	Dashed = 2,
	WideDashed = 3,
}

pub const NO_GRIP: i32 = -1;
// arpeggio etc.
pub const START: i32 = 0;
pub const END: i32 = 1;
pub const MIDDLE: i32 = 2;
// Line
pub const APERTURE: i32 = 3;
// Slur
pub const BEZIER1: i32 = 2;
pub const SHOULDER: i32 = 3;
pub const BEZIER2: i32 = 4;
pub const DRAG: i32 = 5;
// number of grips for slur
pub const GRIPS: i32 = 6;
