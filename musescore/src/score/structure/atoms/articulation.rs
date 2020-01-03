use crate::font::SymName;
use crate::score::{DirectionV, OrnamentStyle};

/// # Articulation
/// articulation marks
#[derive(Debug, Clone)]
pub struct Articulation {
	sym_id: SymName,
	direction: DirectionV,
	channel_name: String,

	anchor: ArticulationAnchor,

	up: bool,
	/// for use in ornaments such as trill
	ornament_style: OrnamentStyle,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum ArticulationAnchor {
	/// anchor is always placed at top of staff
	TopStaff = 0,
	/// anchor is always placed at bottom of staff
	BottomStaff = 1,
	/// anchor depends on chord direction, away from stem
	Chord = 2,
	/// attribute is always placed at top of chord
	TopChord = 3,
	/// attribute is placed at bottom of chord
	BottomChord = 4,
}