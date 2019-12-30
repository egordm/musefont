#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementType {
	Invalid,
	Chord,
	Note,
	Stem,
	Hook,
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