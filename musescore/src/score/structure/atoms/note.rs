use crate::*;
use crate::score::*;
use crate::font::SymName;
use crate::num_traits::FromPrimitive;

pub type Line = i32;
pub type Pitch = i32;

pub const INVALID_LINE: Line = -1;

/// Graphic representation of a note.
#[derive(Debug, Clone)]
pub struct Note {
	element: ElementData,

	sym: SymName,

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
	head_type: NoteheadType,

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

	spanner_for: Vec<SpannerRefWeak>,
	spanner_back: Vec<SpannerRefWeak>,

	/// fingering, other text, symbols or images
	elements: Vec<ElementRef>,
	/// list of note dots (some can be null, read only)
	dots: Vec<El<NoteDot>>,

	pub(crate) cached_notehead_sym: SymName,
}

impl Note {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		sym: SymName::NoSym,
		ghost: false,
		hidden: false,
		dots_hidden: false,
		mirror: false,
		small: false,
		fixed: false,
		user_mirror: DirectionH::Auto,
		user_dot_pos: DirectionV::Auto,
		head_group: NoteheadGroup::Normal,
		head_type: NoteheadType::Auto,
		subchannel: 0,
		line: INVALID_LINE,
		fret: -1,
		string: -1,
		tpc: (Tpc::TpcInvalid, Tpc::TpcInvalid),
		pitch: 0,
		fixed_line: 0,
		tuning: 0.0,
		accidental: None,
		tie_for: None,
		tie_back: None,
		spanner_for: vec![],
		spanner_back: vec![],
		elements: vec![],
		dots: vec![],
		cached_notehead_sym: SymName::NoSym
	})}

	pub fn ghost(&self) -> bool { self.ghost }
	pub fn set_ghost(&mut self, v: bool) { self.ghost = v }
	pub fn hidden(&self) -> bool { self.hidden }
	pub fn set_hidden(&mut self, v: bool) { self.hidden = v }
	pub fn dots_hidden(&self) -> bool { self.dots_hidden }
	pub fn set_dots_hidden(&mut self, v: bool) { self.dots_hidden = v }
	pub fn mirror(&self) -> bool { self.mirror }
	pub fn set_mirror(&mut self, v: bool) { self.mirror = v }
	pub fn small(&self) -> bool { self.small }
	pub fn set_small(&mut self, v: bool) { self.small = v }
	pub fn fixed(&self) -> bool { self.fixed }
	pub fn set_fixed(&mut self, v: bool) { self.fixed = v }

	pub fn user_mirror(&self) -> DirectionH { self.user_mirror }
	pub fn set_user_mirror(&mut self, v: DirectionH) { self.user_mirror = v }
	pub fn user_dot_pos(&self) -> DirectionV { self.user_dot_pos }
	pub fn set_user_dot_pos(&mut self, v: DirectionV) { self.user_dot_pos = v }
	pub fn head_group(&self) -> NoteheadGroup { self.head_group }
	pub fn set_head_group(&mut self, v: NoteheadGroup) { self.head_group = v }
	pub fn head_type(&self) -> NoteheadType { self.head_type }
	pub fn set_head_type(&mut self, v: NoteheadType) { self.head_type = v }

	pub fn subchannel(&self) -> i32 { self.subchannel }
	pub fn set_subchannel(&mut self, v: i32) { self.subchannel = v }
	pub fn line(&self) -> Line { if self.fixed { self.fixed_line } else { self.line } }
	pub fn set_line(&mut self, v: Line) { self.line = v }
	pub fn fret(&self) -> i32 { self.fret }
	pub fn set_fret(&mut self, v: i32) { self.fret = v }
	pub fn string(&self) -> i32 { self.string }
	pub fn set_string(&mut self, v: i32) { self.string = v; self.element.pos.y = self.string as f32 * self.spatium() * 1.5 }
	pub fn tpc(&self) -> Tpc { if self.concert_pitch() { self.tpc.1} else { self.tpc.0 } }
	pub fn set_tpc(&mut self, v: Tpc) { if self.concert_pitch() { self.tpc.1 = v} else { self.tpc.0 = v } }

	pub fn pitch(&self) -> Pitch { self.pitch }
	pub fn set_pitch(&mut self, v: Pitch) { self.pitch = v }

	pub fn fixed_line(&self) -> i32 { self.fixed_line }
	pub fn set_fixed_line(&mut self, v: i32) { self.fixed_line = v }
	pub fn tuning(&self) -> f32 { self.tuning }
	pub fn set_tuning(&mut self, v: f32) { self.tuning = v }

	pub fn accidental(&self) -> &Option<El<Accidental>> { &self.accidental }
	pub fn set_accidental(&mut self, v: Option<El<Accidental>>) { self.accidental = v }

	pub fn tie_for(&self) -> Option<El<Tie>> { self.tie_for.as_ref().and_then(|e| e.upgrade()) }
	pub fn set_tie_for(&mut self, v: Option<ElWeak<Tie>>) { self.tie_for = v }
	pub fn tie_back(&self) -> Option<El<Tie>> { self.tie_back.as_ref().and_then(|e| e.upgrade()) }
	pub fn set_tie_back(&mut self, v: Option<ElWeak<Tie>>) { self.tie_back = v }

	pub fn spanner_for(&self) -> &Vec<SpannerRefWeak> { &self.spanner_for }
	pub fn add_spanner_for(&mut self, v: SpannerRefWeak) { self.spanner_for.push(v) }
	pub fn remove_spanner_for(&mut self, v: &SpannerRefWeak) { remove_element(&mut self.spanner_for, v) }
	pub fn spanner_back(&self) -> &Vec<SpannerRefWeak> { &self.spanner_back }
	pub fn add_spanner_back(&mut self, v: SpannerRefWeak) { self.spanner_back.push(v) }
	pub fn remove_spanner_back(&mut self, v: &SpannerRefWeak) { remove_element(&mut self.spanner_back, v) }

	pub fn elements(&self) -> &Vec<ElementRef> { &self.elements }
	pub fn set_elements(&mut self, v: Vec<ElementRef>) { self.elements = v }

	pub fn dots(&self) -> &Vec<El<NoteDot>> { &self.dots }
	pub fn set_dots(&mut self, v: Vec<El<NoteDot>>) { self.dots = v }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Ghost => self.ghost().into(),
			PropertyId::Small => self.small().into(),
			PropertyId::Fixed => self.fixed().into(),
			PropertyId::MirrorHead => ValueVariant::from_enum(self.user_mirror()),
			PropertyId::DotPosition => ValueVariant::from_enum(self.user_dot_pos()),
			PropertyId::HeadGroup => ValueVariant::from_enum(self.head_group()),
			PropertyId::HeadType => ValueVariant::from_enum(self.head_type()),
			PropertyId::Line => ValueVariant::from_enum(self.line()),
			PropertyId::Fret => self.fret().into(),
			PropertyId::String => self.string().into(),
			PropertyId::Tpc1 => ValueVariant::from_enum(self.tpc.0),
			PropertyId::Tpc2 => ValueVariant::from_enum(self.tpc.1),
			PropertyId::Pitch => ValueVariant::from_enum(self.pitch()),
			PropertyId::FixedLine => self.fixed_line().into(),
			PropertyId::Tuning => self.tuning().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Ghost => v.with_value(|v| self.set_ghost(v)),
			PropertyId::Small => v.with_value(|v| self.set_small(v)),
			PropertyId::Fixed => v.with_value(|v| self.set_fixed(v)),
			PropertyId::MirrorHead => v.with_enum(|v| self.set_user_mirror(v)),
			PropertyId::DotPosition => v.with_enum(|v| self.set_user_dot_pos(v)),
			PropertyId::HeadGroup => v.with_enum(|v| self.set_head_group(v)),
			PropertyId::HeadType => v.with_enum(|v| self.set_head_type(v)),
			PropertyId::Line => v.with_enum(|v| self.set_line(v)),
			PropertyId::Fret => v.with_value(|v| self.set_fret(v)),
			PropertyId::String => v.with_value(|v| self.set_string(v)),
			PropertyId::Tpc1 => v.with_enum(|v| self.tpc.0 = v),
			PropertyId::Tpc2 => v.with_enum(|v| self.tpc.1 = v),
			PropertyId::Pitch => v.with_enum(|v| self.set_pitch(v)),
			PropertyId::FixedLine => v.with_value(|v| self.set_fixed_line(v)),
			PropertyId::Tuning => v.with_value(|v| self.set_tuning(v)),
			_ => false,
		}
	}
}

impl Note {
	/// returns the width of the symbol bbox or the width of the string representation of the fret mark
	pub fn head_width(&self) -> f32 {
		self.score().font().width(self.notehead(), self.scale())
	}
	/// returns the height of the notehead symbol or the height of the string representation of the fret mark
	pub fn head_height(&self) -> f32 { self.score().font().height(self.notehead(), self.scale()) }

	pub fn stem_down_nw(&self) -> Point2F {
		self.score().font().stem_down_nw(self.notehead(), self.scale())
	}
	pub fn stem_up_se(&self) -> Point2F {
		self.score().font().stem_up_se(self.notehead(), self.scale())
	}

	/// returns the x of the symbol bbox. It is different from headWidth() because zero point could be different from leftmost bbox position.
	pub fn bbox_right_pos(&self) -> f32 {
		self.font().bbox(self.notehead(), &size_from(self.scale())).right()
	}
	/// returns the width of the notehead "body". It is actual for slashed noteheads like -O-, where O is body.
	pub fn bbox_body_width(&self) -> f32 {
		self.head_width() + 2. * self.bbox_xshift()
	}
	/// returns the x shift of the notehead bounding box
	pub fn bbox_xshift(&self) -> f32 {
		self.font().bbox(self.notehead(), &size_from(self.scale())).left()
	}
	/// returns the x coordinate of the notehead center related to the basepoint of the notehead bbox
	pub fn notehead_centerx(&self) -> f32 {
		self.head_width() / 2. + self.bbox_xshift()
	}

	/// used only when dropping a notehead from the palette they are either half note,
	/// either double whole
	pub fn notehead_group(&self) -> NoteheadGroup {
		let mut group = NoteheadGroup::Invalid;
		for i in NoteheadGroup::Normal as usize .. NoteheadGroup::DoWalker as usize {
			if NOTE_HEADS[0][i][1] == self.sym || NOTE_HEADS[0][i][3] == self.sym {
				group = NoteheadGroup::from_usize(i).unwrap();
			}
		}
		group
	}
	pub fn notehead(&self) -> SymName   {
		let mut dir = DirectionV::Up;
		let mut head_type = NoteheadType::Quarter;

		if let Some(chord) = self.chord() {
			dir = if chord.borrow_el().up() { DirectionV::Up } else { DirectionV::Down };
			head_type = chord.borrow_el().duration_type().head_type()
		}
		if self.head_type != NoteheadType::Auto { head_type = self.head_type }

		if self.head_group() == NoteheadGroup::Custom {
			if let (Some(_chord), Some(_staff)) = (self.chord(), self.staff()) {
				// Fetch instrument notehead
			} else { return self.cached_notehead_sym; }
		}

		let mut key = Key::C;
		let mut scheme = NoteheadScheme::Normal;
		if let (Some(chord), Some(staff)) = (self.chord(), self.staff()) {
			let tick = chord.borrow_el().time();
			if tick >= Fraction::new(0, 1) {
				let staff = staff.borrow_el();
				key = staff.key(&tick);
				scheme = staff.staff_type(&tick).notehead_scheme();
			}
		}

		let ret = head_type.get_keyed_symid(dir, self.head_group, scheme, self.tpc(), key);
		if SymName::NoSym == ret {
			head_type.get_keyed_symid(dir, NoteheadGroup::Normal, scheme, self.tpc(), key)
		} else {
			ret
		}
	}

	pub fn add(&mut self, e: ElementRef) {
		e.as_trait_mut().attach(self.get_ref(), self.track());

		match e {
			ElementRef::NoteDot(e) => self.dots.push(e),
			ElementRef::Symbol(e) => self.elements.push(e.into()),
			ElementRef::Text(e) => self.elements.push(e.into()),
			ElementRef::Tie(e) => {
				let elt = e.clone();
				let _tie = elt.borrow_mut_el();
				//tie.set_start_note(Some(self.get_ref())) TODO
				//tie.set_track(self.track())
				self.set_tie_for(Some(e.downgrade()));
				//if let Some(en) = tie.end_note() { en.borrow_mut_el().set_tie_back(en) }
			},
			ElementRef::Accidental(e) => self.set_accidental(Some(e)),
			_ => {},
		}
	}
	pub fn remove(&mut self, e: &ElementRef) {
		match e {
			ElementRef::NoteDot(_) => {self.dots.pop();},
			ElementRef::Symbol(_) => remove_element(&mut self.elements, e),
			ElementRef::Text(_) => remove_element(&mut self.elements, e),
			ElementRef::Tie(e) => {
				let _tie = e.borrow_mut_el();
				self.set_tie_for(None);
				//if let Some(en) = tie.end_note() { en.borrow_mut_el().set_tie_back(None) }
			},
			ElementRef::Accidental(_) => self.set_accidental(None),
			_ => {},
		}
	}

	pub fn add_spanner(&mut self, _l: SpannerRef) {
		unimplemented!()
	}
	pub fn remove_spanner(&mut self, _l: SpannerRef) {
		unimplemented!()
	}

	#[allow(irrefutable_let_patterns)]
	pub fn add_parentheses(&mut self) {
		let s = Symbol::new(self.score().clone());
		if let mut s = s.borrow_mut_el() {
			s.set_sym(SymName::NoteheadParenthesisLeft);
			s.set_parent(Some(self.get_ref()));
		}
		self.elements.push(s.into());

		let s = Symbol::new(self.score().clone());
		if let mut s = s.borrow_mut_el() {
			s.set_sym(SymName::NoteheadParenthesisRight);
			s.set_parent(Some(self.get_ref()));
		}
		self.elements.push(s.into());
	}
}

impl Element for Note {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Note }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl AtomTrait for Note {

}
