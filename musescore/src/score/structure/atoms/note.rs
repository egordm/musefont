use crate::score::*;
use crate::font::SymName;

pub type Line = i32;
pub type Pitch = i32;

/// Graphic representation of a note.
#[derive(Debug, Clone)]
pub struct Note {
	element: ElementData,

	/// ghost note (guitar: death note)
	ghost: bool,
	/// marks this note as the hidden one if there are overlapping notes;
	/// hidden notes are not played and heads + accidentals are not shown
	hidden: bool,
	/// dots of hidden notes are hidden too except if only one note is dotted
	dots_hidden: bool,

	/// True if note is mirrored at stem.
	mirror: bool,
	/// small notehead
	small: bool,
	/// for slash notation
	fixed: bool,

	/// user override of mirror
	user_mirror: DirectionH,
	/// user override of dot position
	user_dot_pos: DirectionV,

	head_group: NoteheadGroup,
	head_type: NoteheadGroup,

	/// articulation
	subchannel: i32,
	/// notehead position;
	/// y-Position; 0 - top line.
	line: Line,
	/// for tablature view
	fret: i32,
	/// string number in tablature
	string: i32,

	/// tonal pitch class, as per concert pitch setting
	/// tonal pitch class, non transposed
	/// tonal pitch class, transposed
	tpc: (Tpc, Tpc),
	/// midi pitch
	pitch: Pitch,

	/// fixed line number if _fixed == true
	fixed_line: i32,
	/// pitch offset in cent, playable only by internal synthesizer
	tuning: f32,

	/// note accidental
	accidental: Option<El<Accidental>>,

	tie_for: Option<ElWeak<Tie>>,
	tie_back: Option<ElWeak<Tie>>,

	spanner_for: Option<SpannerRefWeak>,
	spanner_back: Option<SpannerRefWeak>,

	/// fingering, other text, symbols or images
	elements: Vec<ElementRef>,
	/// list of note dots (some can be null, read only)
	dots: Vec<El<NoteDot>>,

	cached_notehead_sym: SymName,
}

impl Element for Note {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Note }
}