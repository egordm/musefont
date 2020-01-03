use crate::score::*;
use crate::Point2F;

/// Example of 1/8 triplet:
///     base_len     = 1/8
///     actual_notes = 3
///     normal_notes = 2   (3 notes played in the time of 2/8)
///
/// The tuplet has a  len of base_len * normal_notes
/// A tuplet note has len of base_len * normal_notes / actual_notes.
#[derive(Debug, Clone)]
pub struct Tuplet {
	elements: Vec<ElementRef>,
	direction: DirectionV,

	number_type: TupletNumberType,
	bracket_type: TupletBracketType,
	bracket_width: Spatium,

	has_bracket: bool,
	ratio: Fraction,
	/// 1/8 for a triplet of 1/8
	base_len: Duration,

	is_up: bool,

	tick: Fraction,

	p1: Point2F,
	p2: Point2F,

	number: Option<Text>,
	bracket_l: [Point2F; 4],
	bracket_r: [Point2F; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TupletNumberType {
	ShowNumber,
	ShowRelation,
	NoText
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TupletBracketType {
	AutoBracket,
	ShowBracket,
	ShowNoBracket
}