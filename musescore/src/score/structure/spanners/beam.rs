use crate::*;
use crate::score::*;

#[derive(Debug, Clone)]
pub struct Beam {
	element: ElementData,

	///  Must be sorted by tick
	chords: OrderedCollecton<ChordRef>,
	segments: Vec<LineF>,
	/// beam splits across systems
	fragments: Vec<BeamFragment>,
	direction: DirectionV,
	user_modified: [bool; 2],

	up: bool,
	/// equal spacing of elements
	distribute: bool,
	no_slope: bool,

	is_grace: bool,
	cross: bool,

	/// define "feather" beams
	grow_left: f32,
	/// define "feather" beams
	grow_right: f32,
	beam_dist: f32,

	min_move: i32,
	max_move: i32,
	max_duration: Duration,
	slope: f32,
}

impl Beam {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),

		chords: OrderedCollecton::new(),
		segments: vec![],
		fragments: vec![],
		direction: DirectionV::Auto,
		user_modified: Default::default(),
		up: true,
		distribute: false,
		no_slope: false,
		is_grace: false,
		cross: false,
		grow_left: 1.0,
		grow_right: 1.0,
		beam_dist: 0.0,
		min_move: 0,
		max_move: 0,
		max_duration: Default::default(),
		slope: 0.0
	})}

	pub fn element_count(&self) -> usize { self.chords.len() }
	pub fn elements(&self) -> impl DoubleEndedIterator<Item=&ChordRef> { self.chords.iter_vals() }

	pub fn has_no_slope(&self) -> bool {
		let idx = match self.direction {
			DirectionV::Auto | DirectionV::Down => 0,
			_ => 1
		};
		self.no_slope && !self.user_modified[idx]
	}

	pub fn chords(&self) -> &OrderedCollecton<ChordRef> { &self.chords }
	pub fn set_chords(&mut self, v: OrderedCollecton<ChordRef>) { self.chords = v }

	pub fn beam_segments(&self) -> &Vec<LineF> { &self.segments }
	pub fn set_segments(&mut self, v: Vec<LineF>) { self.segments = v }
	pub fn fragments(&self) -> &Vec<BeamFragment> { &self.fragments }
	pub fn fragments_mut(&mut self) -> &mut Vec<BeamFragment> { &mut self.fragments }
	pub fn set_fragments(&mut self, v: Vec<BeamFragment>) { self.fragments = v }

	pub fn beam_direction(&self) -> DirectionV { self.direction }
	pub fn set_beam_direction(&mut self, v: DirectionV) { self.direction = v }
	pub fn up(&self) -> bool { self.up }
	pub fn set_up(&mut self, v: bool) { self.up = v }
	pub fn distribute(&self) -> bool { self.distribute }
	pub fn set_distribute(&mut self, v: bool) { self.distribute = v }
	pub fn no_slope(&self) -> bool { self.no_slope }
	pub fn set_no_slope(&mut self, v: bool) { self.no_slope = v }
	pub fn is_grace(&self) -> bool { self.is_grace }
	pub fn set_is_grace(&mut self, v: bool) { self.is_grace = v }
	pub fn cross(&self) -> bool { self.cross }
	pub fn set_cross(&mut self, v: bool) { self.cross = v }

	pub fn grow_left(&self) -> f32 { self.grow_left }
	pub fn set_grow_left(&mut self, v: f32) { self.grow_left = v }
	pub fn grow_right(&self) -> f32 { self.grow_right }
	pub fn set_grow_right(&mut self, v: f32) { self.grow_right = v }

	pub fn beam_dist(&self) -> f32 { self.beam_dist }
	pub fn set_beam_dist(&mut self, v: f32) { self.beam_dist = v }

	pub fn min_move(&self) -> i32 { self.min_move }
	pub fn set_min_move(&mut self, v: i32) { self.min_move = v }
	pub fn max_move(&self) -> i32 { self.max_move }
	pub fn set_max_move(&mut self, v: i32) { self.max_move = v }

	pub fn max_duration(&self) -> &Duration { &self.max_duration }
	pub fn set_max_duration(&mut self, v: Duration) { self.max_duration = v }

	pub fn slope(&self) -> f32 { self.slope }
	pub fn set_slope(&mut self, v: f32) { self.slope = v }

	pub fn beam_pos(&self) -> Point2F {
		if let Some(f) = self.fragments.last() {
			let idx = match self.direction {
				DirectionV::Down | DirectionV::Auto => 0,
				DirectionV::Up => 1,
			};
			f.py[idx] / self.spatium()
		} else { Point2F::default() }
	}

	pub fn set_beam_pos(&mut self, bp: Point2F) {
		let spatium = self.spatium();
		if let Some(f) = self.fragments.last_mut() {
			let idx = match self.direction {
				DirectionV::Down | DirectionV::Auto => 0,
				DirectionV::Up => 1,
			};
			self.user_modified[idx] = true;
			f.py[idx] = bp * spatium
		}
	}

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::StemDirection => ValueVariant::from_enum(self.beam_direction()),
			PropertyId::Distribute => self.distribute().into(),
			PropertyId::GrowLeft => self.grow_left().into(),
			PropertyId::GrowRight => self.grow_right().into(),
			PropertyId::BeamPos => self.beam_pos().into(),
			PropertyId::BeamNoSlope => self.no_slope().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::StemDirection => v.with_enum(|v| self.set_beam_direction(v)),
			PropertyId::Distribute => v.with_value(|v| self.set_distribute(v)),
			PropertyId::GrowLeft => v.with_value(|v| self.set_grow_left(v)),
			PropertyId::GrowRight => v.with_value(|v| self.set_grow_right(v)),
			PropertyId::BeamPos => v.with_value(|v| self.set_beam_pos(v)),
			PropertyId::BeamNoSlope => v.with_value(|v| self.set_no_slope(v)),
			_ => false,
		}
	}

	pub fn add_chord(&mut self, c: ChordRef) {
		let time = c.as_trait().rel_time().ticks();
		self.chords.set(time, c)
	}
}

impl Element for Beam {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Beam }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone())  || self.set_custom_property(p, v)
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpannerSegmentVariant {
	Single,
	Begin,
	Middle,
	End
}

#[derive(Copy, Clone, Debug, PartialEq, Primitive, Eq)]
pub enum BeamMode {
	Auto = 0,
	Begin = 1,
	Mid = 2,
	End = 3,
	None = 4,
	Begin32 = 5,
	Begin64 = 6,
	Invalid = 7,
}

impl BeamMode {
	pub fn no_continue(&self) -> bool {
		match self {
			BeamMode::End | BeamMode::None | BeamMode::Invalid => true,
			_ => false
		}
	}

	pub fn is_mid(&self) -> bool {
		match self {
			BeamMode::Mid | BeamMode::Begin32 | BeamMode::Begin32 => true,
			_ => false
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct BeamFragment {
	pub py: [Point2F; 2],
}