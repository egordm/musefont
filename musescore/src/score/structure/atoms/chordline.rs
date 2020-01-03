use crate::*;
use crate::score::*;

/// # ChordLine
/// bezier line attached to top note of a chord implements fall, doit, plop, bend
#[derive(Debug, Clone)]
pub struct Chordline {
	element: ElementData,

	chord_line_type: ChordLineType,
	straight: bool,
	path: LineF,
	modified: bool,
	length_x: f32,
	length_y: f32,
	initial_length: i32,
}

impl Element for Chordline {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum ChordLineType {
	Notype = 0,
	Fall = 1,
	Doit = 2,
	Plop = 3,
	Scoop = 4,
}