use crate::score::*;
use std::convert::{TryInto, TryFrom};
use crate::remove_element;

#[derive(Debug, Clone)]
pub struct Chord {
	element: ElementData,
	duration_data: DurationElementData,
	rest_data: ChordRestData,

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

impl Chord {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		duration_data: DurationElementData::new(Fraction::default()),
		rest_data: ChordRestData::default(),
		notes: vec![],
		ledger_lines: None,
		stem: None,
		stem_slash: None,
		hook: None,
		arpeggio: None,
		tremolo: None,
		ends_glissando: false,
		grace_notes: vec![],
		grace_index: 0,
		stem_direction: DirectionV::Auto,
		note_type: NoteType::Normal,
		no_stem: false,
		space_lw: 0.0,
		space_rw: 0.0,
		articulations: vec![]
	})}

	pub fn notes(&self) -> &Vec<El<Note>> { &self.notes }
	pub fn set_notes(&mut self, v: Vec<El<Note>>) { self.notes = v }
	pub fn ledger_lines(&self) -> &Option<El<LedgerLine>> { &self.ledger_lines }
	pub fn set_ledger_lines(&mut self, v: Option<El<LedgerLine>>) { self.ledger_lines = v }
	pub fn stem(&self) -> &Option<El<Stem>> { &self.stem }
	pub fn set_stem(&mut self, v: Option<El<Stem>>) { self.stem = v }
	pub fn stem_slash(&self) -> &Option<El<StemSlash>> { &self.stem_slash }
	pub fn set_stem_slash(&mut self, v: Option<El<StemSlash>>) { self.stem_slash = v }
	pub fn hook(&self) -> &Option<El<Hook>> { &self.hook }
	pub fn set_hook(&mut self, v: Option<El<Hook>>) { self.hook = v }
	pub fn arpeggio(&self) -> &Option<ElWeak<Arpeggio>> { &self.arpeggio }
	pub fn set_arpeggio(&mut self, v: Option<ElWeak<Arpeggio>>) { self.arpeggio = v }

	pub fn tremolo(&self) -> &Option<ElWeak<Tremolo>> { &self.tremolo }
	pub fn set_tremolo(&mut self, v: Option<ElWeak<Tremolo>>) { self.tremolo = v }
	pub fn ends_glissando(&self) -> bool { self.ends_glissando }
	pub fn set_ends_glissando(&mut self, v: bool) { self.ends_glissando = v }

	pub fn grace_notes(&self) -> &Vec<El<Chord>> { &self.grace_notes }
	pub fn set_grace_notes(&mut self, v: Vec<El<Chord>>) { self.grace_notes = v }

	pub fn grace_index(&self) -> i32 { self.grace_index }
	pub fn set_grace_index(&mut self, v: i32) { self.grace_index = v }
	pub fn stem_direction(&self) -> DirectionV { self.stem_direction }
	pub fn set_stem_direction(&mut self, v: DirectionV) { self.stem_direction = v }
	pub fn note_type(&self) -> NoteType { self.note_type }
	pub fn set_note_type(&mut self, v: NoteType) { self.note_type = v }
	pub fn no_stem(&self) -> bool { self.no_stem }
	pub fn set_no_stem(&mut self, v: bool) { self.no_stem = v }

	pub fn articulations(&self) -> &Vec<El<Articulation>> { &self.articulations }
	pub fn set_articulations(&mut self, v: Vec<El<Articulation>>) { self.articulations = v }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::NoStem => self.no_stem().into(),
			PropertyId::Small => self.small().into(),
			PropertyId::StemDirection => ValueVariant::from_enum(self.stem_direction()),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::NoStem => v.with_value(|v| self.set_no_stem(v)),
			PropertyId::Small => v.with_value(|v| self.set_small(v)),
			PropertyId::StemDirection => v.with_enum(|v| self.set_stem_direction(v)),
			_ => false,
		}
	}
}

impl Chord {
	pub fn is_grace(&self) -> bool { self.note_type != NoteType::Normal }
	pub fn is_grace_before(&self) -> bool {
		match self.note_type() {
			NoteType::Acciaccatura | NoteType::Appoggiatura | NoteType::Grace4 | NoteType::Grace16 | NoteType::Grace32 => true,
			_ => false
		}
	}
	pub fn is_grace_after(&self) -> bool {
		match self.note_type() {
			NoteType::Grace8After | NoteType::Grace16After | NoteType::Grace32After => true,
			_ => false
		}
	}

	pub fn find_note(&self, pitch: i32, mut skip: i32) -> Option<&El<Note>> {
		self.notes.iter().find(|e| {
			if e.borrow_el().pitch() == pitch {
				if skip == 0 { true } else { skip -= 1; false}
			} else { false }
		})
	}
	pub fn down_note(&self) -> Option<&El<Note>> {
		self.notes.iter().min_by(|a, b| a.borrow_el().line().cmp(&b.borrow_el().line()))
	}
	pub fn up_note(&self) -> Option<&El<Note>> {
		self.notes.iter().max_by(|a, b| a.borrow_el().line().cmp(&b.borrow_el().line()))
	}
	/// Use upstring if tab
	pub fn up_line(&self) -> Line {  self.up_note().map(|e| e.borrow_el().line()).unwrap_or_default() }
	pub fn down_line(&self) -> Line { self.down_note().map(|e| e.borrow_el().line()).unwrap_or_default() }

	pub fn parent_chord(&self) -> Option<ChordRef> {
		self.parent().and_then(|e| ChordRef::try_from(e).ok())
	}

	// TODO: compute up

	// Layout fns stemPos, stemPosBeam, stemPosX
	pub fn under_beam(&self) -> bool {
		if !self.is_grace() { return false; }

		if let Some(chord) = self.parent_chord() {
			if let Some(beam) = chord.as_trait().beam().clone() {
				let beam = beam.borrow_el();
				if !beam.up() { return false; }
				if self.is_grace_before() {
					if beam.elements().next().map(|e| e != &chord).unwrap_or(false)  { return true; }
				} else if self.is_grace_after() {
					if beam.elements().next_back().map(|e| e != &chord).unwrap_or(false) { return true; }
				}
			}
		}
		return false;
	}

	/// Add an element to the Chord
	pub fn add(&mut self, e: ElementRef) { // TODO: move insert logic to actual setters
		e.as_trait_mut().attach(self.get_ref(), self.track());
		match e {
			ElementRef::Note(e) => {
				let mut found = false;

				// notes should be sorted by line position but it's often not yet possible since
				// line is unknown use pitch instead, and line as a second sort criteria.
				let (note_pitch, note_line) = (e.borrow_el().pitch(), e.borrow_el().line());
				for i in 0..self.notes.len() {
					let (other_pitch, other_line) = (self.notes[i].borrow_el().pitch(), self.notes[i].borrow_el().line());
					if note_pitch <= other_pitch {
						if note_pitch == other_pitch && note_line >= other_line {
							self.notes.insert(i + 1, e.clone());
						} else {
							self.notes.insert(i, e.clone());
						}
						found = true;
						break;
					}
				}
				if !found { self.notes.push(e); }

				// TODO: e.borrow_mut_el().connect_tied_notes();
			}
			ElementRef::Arpeggio(e) => self.set_arpeggio(Some(e.downgrade())), // TODO: attach
			ElementRef::Tremolo(e) => self.set_tremolo(Some(e.downgrade())), // TODO: attach
			ElementRef::Stem(e) => self.set_stem(Some(e)),
			ElementRef::StemSlash(e) => self.set_stem_slash(Some(e)),
			ElementRef::Hook(e) => self.set_hook(Some(e)),
			ElementRef::Chordline(_) => self.add_element(e),
			ElementRef::Chord(e) => {
				debug_assert_ne!(e.borrow_el().note_type(), NoteType::Normal);
				let idx = e.borrow_el().grace_index() as usize;
				self.grace_notes.insert(idx, e); // TODO: handle panic
			},
			ElementRef::Articulation(e) => {
				if e.borrow_el().layout_close_to_note() {
					let mut i = 0;
					for ai in &self.articulations {
						if ai.borrow_el().layout_close_to_note() { i += 1; } else {break; }
					}
					self.articulations.insert(i, e);
				} else { self.articulations.push(e); }
			}
			_ => self.add_element(e)
		}
	}
	/// Remove the element from the Chord
	pub fn remove(&mut self, e: &ElementRef) {
		match e {
			ElementRef::Note(e) => {
				if let Some(pos) = self.notes.iter().position(|o| o == e) {
					self.notes.remove(pos);
					let e = e.borrow_mut_el();
					//e.disconnect_tied_notes(); TODO:
					//e.remove_spanners_back()
					//e.remove_spanners_for()
				}
			},
			ElementRef::Arpeggio(_) => self.set_arpeggio(None), // TODO: attach
			ElementRef::Tremolo(_) => self.set_tremolo(None), // TODO: attach
			ElementRef::Stem(_) => self.set_stem(None),
			ElementRef::StemSlash(_) => self.set_stem_slash(None),
			ElementRef::Hook(_) => self.set_hook(None),
			ElementRef::Chordline(_) => self.remove_element(e),
			ElementRef::Chord(e) => remove_element(&mut self.grace_notes, e),
			ElementRef::Articulation(e) => remove_element(&mut self.articulations, e),
			_ => self.remove_element(e)
		}
	}

	/// Return next chord if all notes in this chord are tied to it.
	/// Set backwards=true to return the previous chord instead.
	///
	/// Note: the next chord might have extra notes that are not tied back to this one. Set
	/// sameSize=true to return 0 in this case.
	pub fn next_tied_chord(&self, backwards: bool, same_size: bool) -> Option<El<Chord>> {
		let next_seg = self.segment()?.borrow_el().next_type(SegmentType::Chord)?;
		let next_cr = next_seg.borrow_el().next_chordrest(self.track(), backwards)?;
		if let ChordRef::Chord(chord) = next_cr {
			if same_size && self.notes().len() != chord.borrow_el().notes().len() { return None }
			if self.tuplet() != chord.borrow_el().tuplet() { return None }

			for n in self.notes() {
				let tie = (if backwards { n.borrow_el().tie_back() } else { n.borrow_el().tie_for() })?;
				let nn = (if backwards { tie.borrow_el().start_note() } else { tie.borrow_el().end_note() })?;
				let ch = nn.borrow_el().chord();
				if let ChordRef::Chord(nn_chord) = ch? {
					if nn_chord != chord { return None; }
				} else { return None; }
			}

			return Some(chord);
		}

		return None;
	}

	pub fn to_grace_after(&mut self) {
		match self.note_type() {
			NoteType::Appoggiatura => self.set_note_type(NoteType::Grace8After),
			NoteType::Grace16 => self.set_note_type(NoteType::Grace16After),
			NoteType::Grace32 => self.set_note_type(NoteType::Grace32After),
			_ => {}
		}
	}
}

impl Element for Chord {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Chord }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_chordrest_property(p))
			.if_none(|| self.get_duration_property(p))
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_chordrest_property(p, v.clone())
			|| self.set_duration_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl DurationElement for Chord {
	fn duration_data(&self) -> &DurationElementData { &self.duration_data }
	fn duration_data_mut(&mut self) -> &mut DurationElementData { &mut self.duration_data }
}

impl ChordRestTrait for Chord {
	fn rest_data(&self) -> &ChordRestData { &self.rest_data }
	fn rest_data_mut(&mut self) -> &mut ChordRestData { &mut self.rest_data }
}

impl SegmentTrait for Chord {
	fn segment(&self) -> Option<El<Segment>> {
		self.parent().and_then(|e| {
			if e.as_trait().is_chord() { // If is grace note
				e.as_trait().parent().and_then(|e| e.try_into().ok())
			} else {
				e.try_into().ok()
			}
		})
	}
}

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
