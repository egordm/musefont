use crate::score::*;

#[derive(Debug, Clone)]
pub struct Chord {
	data: Rest,

	/// Sorted to decreasing line step
	notes: Vec<El<Note>>,
	ledger_lines: Option<El<LedgerLine>>,

	stem: Option<El<Stem>>,
	/// For acciacatura
	stem_slash: Option<El<StemSlash>>,
	hook: Option<El<Hook>>,

	arpeggio: Option<ElWeak<Arpeggio>>,
	tremolo: Option<ElWeak<Tremolo>>,

	/// true if this chord is the ending point of a glissando (needed for layout)
	ends_glissando: bool,
	grace_notes: Vec<El<Chord>>,
	/// if this is a grace note, index in parent list
	grace_index: i32,

	stem_direction: DirectionV,
	/// mark grace notes: acciaccatura and appoggiatura
	note_type: NoteType,
	no_stem: bool,

	space_lw: f32,
	space_rw: f32,

	articulations: Vec<El<Articulation>>,
}

impl SegmentTrait for Chord {}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum NoteType {
	Normal = 0,
	Acciaccatura = 0x1,
	Appoggiatura = 0x2,
	// grace notes
	Grace4 = 0x4,
	Grace16 = 0x8,
	Grace32 = 0x10,
	Grace8After = 0x20,
	Grace16After = 0x40,
	Grace32After = 0x80,
	Invalid = 0xFF
}
