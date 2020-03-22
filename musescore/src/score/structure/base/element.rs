use crate::*;
use crate::score::*;
use crate::drawing::PainterRef;

pub type Track = u16;
pub type Voice = u16;
pub type StaffId = u16;
pub const MAX_STAFF_ID: StaffId = std::u16::MAX;
pub type ElementStyle = (PropertyId, StyleName);

pub fn new_element<T: Element + Clone>(e: T) -> El<T>
	where ElementRef: From<El<T>>
{
	let ret = El::from(e);
	let self_ref = ElementRef::from(ret.clone()).downgrade();
	ret.borrow_mut_el().set_ref(self_ref);
	return ret;
}

#[derive(Clone, Debug)]
pub struct ElementData {
	score_element: ScoreElementData,
	/// Bounding box relative to _pos + _offset
	bbox: RectF,
	/// standard magnification (derived value)
	scale: f32,
	/// Reference position, relative to _parent, set by autoplace
	pub(crate) pos: Point2F,
	/// offset from reference position, set by autoplace or user
	offset: Point2F,
	/// autoplace min distance
	min_dist: Spatium,
	/// staffIdx * VOICES + voice
	track: Track,
	flags: ElementFlags,
}

impl ElementData {
	pub fn new(score: Score)  -> Self {Self {
		score_element: ScoreElementData::new(score),
		bbox: Default::default(),
		scale: 1.0,
		pos: Default::default(),
		offset: Default::default(),
		min_dist: Spatium(0.0),
		track: 0,
		flags: ElementFlags::NOTHING
	}}
}

pub trait Element: ScoreElement {
	fn el_data(&self) -> &ElementData;
	fn el_data_mut(&mut self) -> &mut ElementData;

	fn element_type(&self) -> ElementType;

	fn attach(&mut self, parent: ElementRefWeak, track: Track) {
		self.set_parent(Some(parent));
		self.set_track(track);
	}

	// Positon and scale properties
	fn ipos(&self) -> &Point2F { &self.el_data().pos }
	fn pos(&self) -> Point2F { self.el_data().pos + self.el_data().offset.to_vector() }
	fn x(&self) -> f32 { self.el_data().pos.x + self.el_data().offset.x }
	fn y(&self) -> f32 { self.el_data().pos.y + self.el_data().offset.y }
	fn set_pos(&mut self, pos: Point2F) { self.el_data_mut().pos = pos; }
	fn move_pos(&mut self, dt: &Point2F) { self.el_data_mut().pos += dt.to_vector(); }

	fn scale(&self) -> f32 { self.el_data().scale }
	fn set_scale(&mut self, scale: f32) { self.el_data_mut().scale = scale; }

	fn offset(&self) -> &Point2F { &self.el_data().offset }
	fn set_offset(&mut self, v: Point2F) { self.el_data_mut().offset = v; }

	fn bbox(&self) -> &RectF { &self.el_data().bbox }
	fn set_bbox(&mut self, v: RectF) { self.el_data_mut().bbox = v; }
	fn add_bbox(&mut self, v: &RectF) { self.el_data_mut().bbox = self.el_data_mut().bbox.union(v); }
	fn width(&self) -> f32 { self.el_data().bbox.size.width }
	fn set_width(&mut self, v: f32) { self.el_data_mut().bbox.size.width = v; }
	fn height(&self) -> f32 { self.el_data().bbox.size.height }
	fn set_height(&mut self, v: f32) { self.el_data_mut().bbox.size.height = v; }
	fn contains(&self, p: &Point2F) -> bool { self.el_data().bbox.contains(*p) }
	fn intersects(&self, r: &RectF) -> bool { self.el_data().bbox.intersects(r) }

	// Flags
	fn flag(&self, f: ElementFlags) -> bool { self.el_data().flags.bits & f.bits == f.bits }
	fn set_flag(&mut self, f: ElementFlags, v: bool) { self.el_data_mut().flags.set(f, v) }

	fn system_flag(&self) -> bool {self.flag(ElementFlags::SYSTEM)}
	fn set_system_flag(&mut self, v: bool) {self.set_flag(ElementFlags::SYSTEM, v)}
	fn header(&self) -> bool {self.flag(ElementFlags::HEADER)}
	fn set_header(&mut self, v: bool) {self.set_flag(ElementFlags::HEADER, v)}
	fn trailer(&self) -> bool {self.flag(ElementFlags::TRAILER)}
	fn set_trailer(&mut self, v: bool) {self.set_flag(ElementFlags::TRAILER, v)}
	fn visible(&self) -> bool { !self.flag(ElementFlags::INVISIBLE) }
	fn set_visible(&mut self, v: bool) {self.set_flag(ElementFlags::INVISIBLE, !v)}
	fn selected(&self) -> bool { self.flag(ElementFlags::SELECTED) }
	fn set_selected(&mut self, v: bool) {self.set_flag(ElementFlags::SELECTED, v)}
	fn generated(&self) -> bool { self.flag(ElementFlags::GENERATED) }
	fn set_generated(&mut self, v: bool) {self.set_flag(ElementFlags::GENERATED, v)}
	fn autoplace(&self) -> bool {
		self.score().style().value_bool(StyleName::AutoplaceEnabled)
			&& !self.flag(ElementFlags::NO_AUTOPLACE)
	}
	fn set_autoplace(&mut self, v: bool) {self.set_flag(ElementFlags::NO_AUTOPLACE, !v)}
	fn placement(&self) -> Placement { if self.flag(ElementFlags::PLACE_ABOVE) { Placement::Above } else { Placement::Below } }
	fn set_placement(&mut self, p: Placement) {self.set_flag(ElementFlags::PLACE_ABOVE, p == Placement::Above)}
	fn size_is_spatium_dependent(&self) -> bool {!self.flag(ElementFlags::SIZE_SPATIUM_DEPENDENT)}
	fn set_size_is_spatium_dependent(&mut self, v: bool) {self.set_flag(ElementFlags::SIZE_SPATIUM_DEPENDENT, !v)}

	// Score properties
	fn track(&self) -> Track { self.el_data().track }
	fn set_track(&mut self, v: Track) { self.el_data_mut().track = v }
	fn staff_id(&self) -> StaffId { self.track() >> 2 }
	fn voice(&self) -> Voice { self.track() & 3 }
	fn set_voice(&mut self, v: Voice) { self.set_track((self.track() / constants::VOICES as Track) * constants::VOICES as Track + v) }

	fn time(&self) -> Fraction {
		let mut iter = self.parent_iter();
		while let Some(e) = iter.next() {
			if e.as_trait().is_segment() || e.as_trait().is_measure() {
				return e.as_trait().time();
			}
		}
		return Fraction::new(0, 1);
	}

	fn baseline(&self) -> f32 { -self.height() }

	// Score main elements
	fn staff(&self) -> Option<El<Staff>> {
		self.score().staff(self.staff_id())
	}
	fn part(&self) -> Option<El<Part>> {
		self.staff()?.borrow_el().part().clone().and_then(|e| e.upgrade())
	}

	// Properties
	fn load_properties(&mut self, es: &[ElementStyle]) {
		for (_property, _style_name) in es {
			//self.set_property(property, )
			unimplemented!()
		}
	}

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_element_property(p)
	}
	fn get_element_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Tick => self.time().ticks().into(),
			PropertyId::Track => (self.track() as u32).into(),
			PropertyId::Voice => (self.voice() as u32).into(),
			PropertyId::Position => self.pos().into(),
			PropertyId::Visible => self.visible().into(),
			PropertyId::Selected => self.selected().into(),
			PropertyId::Offset => self.offset().into(),
			PropertyId::MinDistance => self.el_data().min_dist.into(), // TODO: spatium type
			PropertyId::Placement => (self.placement() as u32).into(),
			PropertyId::Autoplace => self.autoplace().into(),
			PropertyId::SystemFlag => self.system_flag().into(),
			PropertyId::SizeSpatiumDependent => self.size_is_spatium_dependent().into(),
			_ => ValueVariant::None,
		}
	}

	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v)
	}
	fn set_element_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Track => v.with_value(|v: u32| self.set_track(v as Track)),
			PropertyId::Voice => v.with_value(|v: u32| self.set_voice(v as Voice)),
			PropertyId::Position => v.with_value(|v| self.set_pos(v)),
			PropertyId::Visible => v.with_value(|v| self.set_visible(v)),
			PropertyId::Selected => v.with_value(|v| self.set_selected(v)),
			PropertyId::Offset => v.with_value(|v| self.set_offset(v)),
			PropertyId::MinDistance => v.with_value(|v| self.el_data_mut().min_dist = v),
			PropertyId::Placement => v.with_enum(|v| self.set_placement(v)),
			PropertyId::Autoplace => v.with_value(|v| self.set_autoplace(v)),
			PropertyId::SystemFlag => v.with_value(|v| self.set_system_flag(v)),
			PropertyId::SizeSpatiumDependent => v.with_value(|v| self.set_size_is_spatium_dependent(v)),
			_ => false,
		}
	}

	// Style
	fn concert_pitch(&self) -> bool {
		self.score().style().value_bool(StyleName::ConcertPitch)
	}

	// Typing
	fn is_segment(&self) -> bool { is_segment(self.element_type()) }
	fn is_measure(&self) -> bool { is_measure(self.element_type()) }
	fn is_spanner(&self) -> bool { is_spanner(self.element_type()) }
	fn is_chord(&self) -> bool { is_chord(self.element_type()) }

	fn spatium(&self) -> f32 {
		if self.system_flag() {
			self.score().spatium()
		} else {
			if let Some(staff) = self.staff() {
				staff.borrow_el().spatium(&self.time())
			} else {
				self.score().spatium()
			}
		}
	}

	fn layout(e: El<Self>) where Self: Sized {
		unimplemented!() // TODO: dont default here. Require implementaton
	}

	fn render(e: El<Self>, state: &mut RendererState, painter: PainterRef) where Self: Sized {
		unimplemented!() // TODO: dont default here. Require implementaton
	}
}

impl<T: Element> ScoreElement for T {
	fn score_data(&self) -> &ScoreElementData { &self.el_data().score_element }
	fn score_data_mut(&mut self) -> &mut ScoreElementData { &mut self.el_data_mut().score_element }
}

bitflags! { pub struct ElementFlags: u32 {
	const NOTHING                = 0x00000000;
	const DROP_TARGET            = 0x00000001;
	const NOT_SELECTABLE         = 0x00000002;
	const MOVABLE                = 0x00000004;
	const COMPOSITION            = 0x00000008;       // true if element is part of another element
	const HAS_TAG                = 0x00000010;       // true if this is a layered element
	const ON_STAFF               = 0x00000020;
	const SELECTED               = 0x00000040;
	const GENERATED              = 0x00000080;
	const INVISIBLE              = 0x00000100;
	const NO_AUTOPLACE           = 0x00000200;
	const SYSTEM                 = 0x00000400;
	const PLACE_ABOVE            = 0x00000800;
	const SIZE_SPATIUM_DEPENDENT = 0x00001000;

	// measure flags
	const REPEAT_END             = 0x00002000;
	const REPEAT_START           = 0x00004000;
	const REPEAT_JUMP            = 0x00008000;
	const IRREGULAR              = 0x00010000;
	const LINE_BREAK             = 0x00020000;
	const PAGE_BREAK             = 0x00040000;
	const SECTION_BREAK          = 0x00080000;
	const NO_BREAK               = 0x00100000;
	const HEADER                 = 0x00200000;
	const TRAILER                = 0x00400000;    // also used in segment
	const KEYSIG                 = 0x00800000;

	// segment flags
	const ENABLED                = 0x01000000;    // used for segments
	const EMPTY                  = 0x02000000;
	const WRITTEN                = 0x04000000;
}}