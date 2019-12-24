#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementType {
	Invalid,
	Note,
	Stem,
	Clef,
	Rest,
	Tie,
	Beam,
	NoteHead,
	NoteDot,
	Symbol,
	Group,
	Accidental,
}