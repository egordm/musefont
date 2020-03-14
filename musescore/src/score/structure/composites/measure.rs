use log::{debug};
use crate::*;
use crate::score::*;
use std::convert::TryInto;
use bitflags::_core::ops::RangeBounds;

/// One measure in a system
#[derive(Debug, Clone)]
pub struct Measure {
	element: ElementData,

	mstaves: Vec<MStaff>,
	measure_data: MeasureData,
	segments: SegmentMap,

	/// > 0 if this is a multi measure rest
	/// 0 if this is the start of a mm rest (_mmRest != 0)
	/// < 0 if this measure is covered by a mm rest
	mm_rest_count: i32,
	/// multi measure rest which replaces a measure range
	mm_rest: Option<MeasureRefWeak>,
	break_mm_rest: bool,

	timesig: Fraction,
	/// end repeat marker und repeat count
	repeat_count: i32,
	user_stretch: f32,

	no_mode: MeasureNumberMode,
}

impl Measure {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		mstaves: vec![],
		measure_data: Default::default(),
		segments: SegmentMap::new(),
		mm_rest_count: 0,
		mm_rest: None,
		timesig: Fraction::new(4, 4),
		repeat_count: 0,
		user_stretch: 0.0,
		no_mode: MeasureNumberMode::Auto,
		break_mm_rest: false
	})}

	pub fn segments(&self) -> &SegmentMap { &self.segments }

	pub fn mm_rest(&self) -> Option<MeasureRef> { self.mm_rest.as_ref()?.upgrade() }
	pub fn set_mm_rest(&mut self, v: Option<MeasureRefWeak>) { self.mm_rest = v }

	pub fn timesig(&self) -> &Fraction { &self.timesig }
	pub fn set_timesig(&mut self, v: Fraction) { self.timesig = v }
	pub fn repeat_count(&self) -> i32 { self.repeat_count }
	pub fn set_repeat_count(&mut self, v: i32) { self.repeat_count = v }

	pub fn no_mode(&self) -> MeasureNumberMode { self.no_mode }
	pub fn set_no_mode(&mut self, v: MeasureNumberMode) { self.no_mode = v }

	pub fn has_voices(&self, staff_id: StaffId) -> bool { self.mstaves[staff_id as usize].has_voices() }
	pub fn set_has_voices(&mut self, staff_id: StaffId, v: bool) { self.mstaves[staff_id as usize].set_has_voices(v) }
	pub fn lines(&self, staff_id: StaffId) -> &Option<El<StaffLines>> { self.mstaves[staff_id as usize].lines() }
	pub fn set_lines(&mut self, staff_id: StaffId, v: Option<El<StaffLines>>) { self.mstaves[staff_id as usize].set_lines(v) }

	pub fn vspacer_up(&self, staff_id: StaffId) -> &Option<El<Spacer>> { self.mstaves[staff_id as usize].vspacer_up() }
	pub fn vspacer_down(&self, staff_id: StaffId) -> &Option<El<Spacer>> { self.mstaves[staff_id as usize].vspacer_down() }

	pub fn staff_visible(&self, staff_id: StaffId) -> bool { self.mstaves[staff_id as usize].visible() }
	pub fn set_staff_visible(&mut self, staff_id: StaffId, v: bool) { self.mstaves[staff_id as usize].set_visible(v) }
	pub fn stemless(&self, staff_id: StaffId) -> bool { self.mstaves[staff_id as usize].stemless() }
	pub fn set_staff_stemless(&mut self, staff_id: StaffId, v: bool) { self.mstaves[staff_id as usize].set_stemless(v) }

	pub fn is_irregular(&self) -> bool { self.timesig != self.duration() }
	pub fn stretched_len(&self, staff: &El<Staff>) -> Fraction {
		self.duration() * staff.borrow_el().timestretch(&self.time())
	}

	pub fn create_staves(&mut self, staff_id: StaffId) {
		for n in self.mstaves.len()..staff_id as usize + 1 {
			if let Some(staff) = self.score().staff(n as StaffId) {
				let mut s = MStaff::default();
				let lines = StaffLines::new(self.score().clone());
				s.set_lines(Some(lines.clone()));
				lines.borrow_mut_el().attach(self.get_ref(), self.track());
				lines.borrow_mut_el().set_visible(!staff.borrow_el().invisible());
				self.mstaves.push(s);
			}
		}
	}

	pub fn user_stretch(&self) -> f32 {
		//if self.score().layout_mode() == LayoutMode::FLOAT { 1.0 } else { self.user_stretch }
		1.0
	}

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::TimesigNominal => self.timesig.ticks().into(),
			PropertyId::TimesigActual => self.duration().ticks().into(),
			PropertyId::MeasureNumberMode => ValueVariant::from_enum(self.no_mode()),
			PropertyId::BreakMmr => self.break_mm_rest.into(),
			PropertyId::RepeatCount => self.repeat_count().into(),
			PropertyId::UserStretch => self.user_stretch().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::TimesigNominal => v.with_value(|v| self.timesig = Fraction::from_ticks(v)),
			PropertyId::TimesigActual => v.with_value(|v| self.set_duration(Fraction::from_ticks(v))),
			PropertyId::MeasureNumberMode => v.with_enum(|v| self.set_no_mode(v)),
			PropertyId::BreakMmr => v.with_value(|v | self.break_mm_rest = v),
			PropertyId::RepeatCount => v.with_value(|v| self.repeat_count = v),
			PropertyId::UserStretch => v.with_value(|v| self.user_stretch = v),
			_ => false,
		}
	}

	pub fn segment_iter_range(&self, r: impl RangeBounds<Fraction>) -> impl DoubleEndedIterator<Item=&El<Segment>> {
		self.segments.range(r).map(|(_, v)| v)
	}

	pub fn segment_next_iter(&self, time: Fraction) -> impl DoubleEndedIterator<Item=&El<Segment>> {
		self.segment_iter_range(time..)
	}
	pub fn segment_prev_iter(&self, time: Fraction) -> impl DoubleEndedIterator<Item=&El<Segment>> {
		self.segment_iter_range(..Fraction::from_ticks(time.ticks() + 1)).rev()
	}

	/// Search for chord at position \a tick in \a track
	pub fn find_chord(&self, t: Fraction, track: Track) -> Option<El<Chord>> {
		for seg in self.segments.iter_ty(t - self.time(), SegmentType::Chord).map(|(_, v)| v) {
			if let Some(ElementRef::Chord(chord)) = seg.borrow_el().element(track) {
				return Some(chord.clone());
			}
		}
		return None;
	}
	/// Search for chord or rest at position \a tick at \a staff in \a voice.
	pub fn find_chordrest(&self, t: Fraction, track: Track) -> Option<ChordRef> {
		for seg in self.segments.iter_ty(t - self.time(), SegmentType::Chord).map(|(_, v)| v) {
			if let Some(v) = seg.borrow_el().element(track).and_then(|e| e.clone().try_into().ok()) {
				return Some(v);
			}
		}
		return None;
	}
	pub fn segment_at(&self, t: Fraction, ty: SegmentType) -> Option<El<Segment>> {
		self.segments.get_ty(t - self.time(), ty).cloned()
	}
	/// Search for a segment of type st at measure relative position t.
	pub fn find_segment_r(&self, t: Fraction, st: SegmentType) -> Option<El<Segment>> {
		self.segment_at(t + self.time(), st)
	}
	/// Get a segment of type st at relative tick position t.
	/// If the segment does not exist, it is created.
	pub fn get_segment_r(&mut self, t: Fraction, st: SegmentType) -> El<Segment> {
		if let Some(s) = self.find_segment_r(t, st) { s }
		else {
			let s = Segment::new(self.score().clone());
			{
				let mut r = s.borrow_mut_el();
				r.attach(self.get_ref(), self.track());
				r.set_segment_type(st);
				r.set_rel_time(t);
			}
			self.add(s.clone().into());
			return s;
		}
	}

	pub fn insert_staves(&mut self, s_staff: StaffId, e_staff: StaffId) {
		for e in self.elements() {
			let mut staff_id = e.as_trait().staff_id();
			if staff_id >= s_staff && !e.as_trait().system_flag() {
				let voice = e.as_trait().voice();
				staff_id += e_staff - s_staff;
				e.as_trait_mut().set_track(staff_id * constants::VOICES as StaffId + voice)
			}
		}
		for s in self.segments().iter_vals() {
			for staff in s_staff..e_staff {
				s.borrow_mut_el().insert_staff(staff);
			}
		}
	}
}

impl MeasureTrait for Measure {
	fn measure_data(&self) -> &MeasureData { &self.measure_data }
	fn measure_data_mut(&mut self) -> &mut MeasureData {&mut self.measure_data }

	fn add(&mut self, e: ElementRef) {
		match e {
			ElementRef::Segment(e) => {
				let t = e.borrow_el().rel_time();
				let st = e.borrow_el().segment_type();
				for other in self.segment_next_iter(t) {
					if other.borrow_el().rel_time() != t { break; }
					if other.borrow_el().segment_type() == st {
						debug!("Segment with type {} already exists in this measure!", st.bits());
						return;
					}
				}
				self.segments.insert(e.clone());

				if e.borrow_el().header() { self.set_header(true); }
				if e.borrow_el().trailer() { self.set_trailer(true); }
			}
			//ElementRef::MeasureNumber(e) => {}
			ElementRef::Spacer(e) => {
				let staff_id = e.borrow_el().staff_id() as usize;
				let st = e.borrow_el().spacer_type();
				match st { // Spacer relayout trigger
					SpacerType::Up => self.mstaves[staff_id].set_vspacer_up(Some(e)),
					SpacerType::Down | SpacerType::Fixed => self.mstaves[staff_id].set_vspacer_down(Some(e)),
				}
			}
			ElementRef::HBox(e) => self.add_element(e.into()),
			ElementRef::Measure(e) => self.mm_rest = Some(MeasureRef::Measure(e).downgrade()),
			_ => self.base_add(e)
		}
	}

	fn remove(&mut self, e: &ElementRef) {
		match e {
			ElementRef::Segment(e) => {
				self.segments.remove(e);
				if e.borrow_el().header() {
					// TODO: try unset
				}
				if e.borrow_el().trailer() {
					// TODO: try unset
				}
			},
			ElementRef::Spacer(e) => {
				let staff_id = e.borrow_el().staff_id() as usize;
				let st = e.borrow_el().spacer_type();
				match st { // Spacer relayout trigger
					SpacerType::Up => self.mstaves[staff_id].set_vspacer_up(None),
					SpacerType::Down | SpacerType::Fixed => self.mstaves[staff_id].set_vspacer_down(None),
				}
			}
			ElementRef::HBox(_) => self.remove_element(&e),
			ElementRef::Clef(_) | ElementRef::Chord(_) | ElementRef::Rest(_) | ElementRef::TimeSig(_) => {
				let track = e.as_trait().track();
				for s in self.segments.iter_vals() {
					if s.borrow_el().element(track) == Some(e) {
						s.borrow_mut_el().set_element(track, None);
						return;
					}
				}
			},
			ElementRef::Measure(_) => self.mm_rest = None,
			_ => self.base_remove(e)
		}
	}
}

impl Element for Measure {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Measure }

	fn time(&self) -> Fraction { self.measure_data.time }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_measure_property(p))
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_measure_property(p, v.clone())
			|| self.set_custom_property(p, v)
	}
}

#[derive(Debug, Clone, Copy, Primitive)]
pub enum MeasureNumberMode {
	/// show measure number depending on style
	Auto = 0,
	/// always show measure number
	Show = 1,
	/// don’t show measure number
	Hide = 2,
}

#[derive(Debug, Clone)]
pub struct MStaff {
	// Measure number text object
//	measure_number: MeasureNumber,
	lines: Option<El<StaffLines>>,
	vspacer_up: Option<El<Spacer>>,
	vspacer_down: Option<El<Spacer>>,

	/// indicates that MStaff contains more than one voice,
	/// this changes some layout rules
	has_voices: bool,
	visible: bool,
	stemless: bool,
}

impl Default for MStaff {
	fn default() -> Self {Self{
		lines: None,
		vspacer_up: None,
		vspacer_down: None,
		/// indicates that MStaff contains more than one voice, this changes some layout rules
		has_voices: false,
		visible: true,
		stemless: false
	}}
}

impl MStaff {
	pub fn lines(&self) -> &Option<El<StaffLines>> { &self.lines }
	pub fn set_lines(&mut self, v: Option<El<StaffLines>>) { self.lines = v }

	pub fn vspacer_up(&self) -> &Option<El<Spacer>> { &self.vspacer_up }
	pub fn set_vspacer_up(&mut self, v: Option<El<Spacer>>) { self.vspacer_up = v }
	pub fn vspacer_down(&self) -> &Option<El<Spacer>> { &self.vspacer_down }
	pub fn set_vspacer_down(&mut self, v: Option<El<Spacer>>) { self.vspacer_down = v }

	pub fn has_voices(&self) -> bool { self.has_voices }
	pub fn set_has_voices(&mut self, v: bool) { self.has_voices = v }

	pub fn visible(&self) -> bool { self.visible }
	pub fn set_visible(&mut self, v: bool) { self.visible = v }
	pub fn stemless(&self) -> bool { self.stemless }
	pub fn set_stemless(&mut self, v: bool) { self.stemless = v }
}