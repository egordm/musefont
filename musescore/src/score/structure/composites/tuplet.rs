use crate::score::*;
use crate::{Point2F, remove_element};
use std::convert::TryInto;

/// Example of 1/8 triplet:
///     base_len     = 1/8
///     actual_notes = 3
///     normal_notes = 2   (3 notes played in the time of 2/8)
///
/// The tuplet has a  len of base_len * normal_notes
/// A tuplet note has len of base_len * normal_notes / actual_notes.
#[derive(Debug, Clone)]
pub struct Tuplet {
	element: ElementData,
	duration_data: DurationElementData,

	elements: Vec<DurationElementRef>,
	direction: DirectionV,

	number_type: TupletNumberType,
	bracket_type: TupletBracketType,
	bracket_width: Spatium,

	has_bracket: bool,
	ratio: Fraction,
	/// 1/8 for a triplet of 1/8
	base_len: Duration,

	is_up: bool,

	time: Fraction,

	p1: Point2F,
	p2: Point2F,

	number: Option<El<Text>>,
	bracket_l: [Point2F; 4],
	bracket_r: [Point2F; 3],
}

impl Tuplet {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		duration_data: DurationElementData::new(Fraction::new(1, 1)),
		elements: vec![],
		direction: DirectionV::Down,
		number_type: TupletNumberType::ShowNumber,
		bracket_type: TupletBracketType::AutoBracket,
		bracket_width: Spatium(0.0),
		has_bracket: false,
		ratio: Default::default(),
		base_len: Default::default(),
		is_up: true,
		time: Default::default(),
		p1: Default::default(),
		p2: Default::default(),
		number: None,
		bracket_l: Default::default(),
		bracket_r: Default::default()
	})}

	pub fn elements(&self) -> &Vec<DurationElementRef> { &self.elements }
	pub fn set_elements(&mut self, v: Vec<DurationElementRef>) { self.elements = v }

	pub fn contains(&self, e: &DurationElementRef) -> bool { self.elements.contains(e) }

	pub fn direction(&self) -> DirectionV { self.direction }
	pub fn set_direction(&mut self, v: DirectionV) { self.direction = v }

	pub fn number_type(&self) -> TupletNumberType { self.number_type }
	pub fn set_number_type(&mut self, v: TupletNumberType) { self.number_type = v }
	pub fn number(&self) -> Option<&El<Text>> { self.number.as_ref() }
	pub fn set_number(&mut self, v: Option<El<Text>>) { self.number = v }

	pub fn bracket_type(&self) -> TupletBracketType { self.bracket_type }
	pub fn set_bracket_type(&mut self, v: TupletBracketType) { self.bracket_type = v }
	pub fn bracket_width(&self) -> Spatium { self.bracket_width }
	pub fn set_bracket_width(&mut self, v: Spatium) { self.bracket_width = v }
	pub fn has_bracket(&self) -> bool { self.has_bracket }
	pub fn set_has_bracket(&mut self, v: bool) { self.has_bracket = v }
	pub fn bracket_l(&self) -> &[Point2F; 4] { &self.bracket_l }
	pub fn set_bracket_l(&mut self, v: [Point2F; 4]) { self.bracket_l = v }
	pub fn bracket_r(&self) -> &[Point2F; 3] { &self.bracket_r }
	pub fn set_bracket_r(&mut self, v: [Point2F; 3]) { self.bracket_r = v }

	pub fn ratio(&self) -> &Fraction { &self.ratio }
	pub fn set_ratio(&mut self, v: Fraction) { self.ratio = v }
	pub fn base_len(&self) -> &Duration { &self.base_len }
	pub fn set_base_len(&mut self, v: Duration) { self.base_len = v }
	pub fn is_up(&self) -> bool { self.is_up }
	pub fn set_is_up(&mut self, v: bool) { self.is_up = v }

	pub fn rel_time(&self) -> Fraction { self.time - self.measure().map(|m| m.as_trait().time()).unwrap_or(Fraction::new(0, 0)) }
	pub fn set_time(&mut self, v: Fraction) { self.time = v }

	pub fn measure(&self) -> Option<MeasureRef> { self.parent().and_then(|e| e.try_into().ok()) }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Direction => ValueVariant::from_enum(self.direction()),
			PropertyId::NumberType => ValueVariant::from_enum(self.number_type()),
			PropertyId::BracketType => ValueVariant::from_enum(self.bracket_type()),
			PropertyId::LineWidth => self.bracket_width().into(),
			PropertyId::NormalNotes => self.ratio().den().into(),
			PropertyId::ActualNotes => self.ratio().num().into(),
			PropertyId::P1 => self.p1.into(),
			PropertyId::P2 => self.p2.into(),
			PropertyId::FontSize | PropertyId::FontFace | PropertyId::FontStyle | PropertyId::Align
			| PropertyId::SizeSpatiumDependent => self.number.as_ref().map(|n| n.borrow_el().get_element_property(p)).unwrap_or_default(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Direction => v.with_enum(|v| self.set_direction(v)),
			PropertyId::NumberType => v.with_enum(|v| self.set_number_type(v)),
			PropertyId::BracketType => v.with_enum(|v| self.set_bracket_type(v)),
			PropertyId::LineWidth => v.with_value(|v| self.set_bracket_width(v)),
			PropertyId::NormalNotes => v.with_value(|v| self.ratio.denominator = v),
			PropertyId::ActualNotes => v.with_value(|v| self.ratio.numerator = v),
			PropertyId::P1 => v.with_value(|v| self.p1 = v),
			PropertyId::P2 => v.with_value(|v| self.p2 = v),
			PropertyId::FontSize | PropertyId::FontFace | PropertyId::FontStyle | PropertyId::Align
			| PropertyId::SizeSpatiumDependent => self.number().map(|n| n.borrow_mut_el().set_element_property(p, v)).unwrap_or_default(),
			_ => false,
		}
	}

	pub fn add(&mut self, e: ElementRef) {
		if let Ok(e) = TryInto::<DurationElementRef>::try_into(e) {
			if self.contains(&e) { return; }

			let time = e.as_trait().time();
			let pos = self.elements.iter().position(|e| e.as_trait().time() > time);
			match  pos {
				None => { self.elements.push(e); },
				Some(pos) => {self.elements.insert(pos, e)},
			}
		}
	}

	pub fn remove(&mut self, e: &ElementRef) {
		if let Ok(e) = TryInto::<DurationElementRef>::try_into(e.clone()) {
			remove_element(&mut self.elements, &e)
		}
	}
}

impl Element for Tuplet {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Tuplet }

	fn time(&self) -> Fraction { self.time }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_duration_property(p))
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_duration_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl DurationElement for Tuplet {
	fn duration_data(&self) -> &DurationElementData { &self.duration_data}
	fn duration_data_mut(&mut self) -> &mut DurationElementData { &mut self.duration_data }


}

#[derive(Clone, Copy, Debug, PartialEq, Primitive, Eq, Hash)]
pub enum TupletNumberType {
	ShowNumber = 0,
	ShowRelation =1,
	NoText = 2
}

#[derive(Clone, Copy, Debug, PartialEq, Primitive, Eq, Hash)]
pub enum TupletBracketType {
	AutoBracket = 0,
	ShowBracket = 1,
	ShowNoBracket = 2
}