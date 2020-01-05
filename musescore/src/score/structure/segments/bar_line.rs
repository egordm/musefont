use crate::score::*;
use crate::remove_element;

#[derive(Debug, Clone)]
pub struct Barline {
	element: ElementData,

	/// span barline to next staff if true, values > 1 are used for importing from 2.x
	span_staff: i32,
	/// line number on start and end staves
	span_from: i32,
	span_to: i32,
	bar_line_type: BarLineType,

	y1: f32,
	y2: f32,

	/// fermata or other articulations
	elements: Vec<ElementRef>,
}

impl Barline {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		span_staff: 0,
		span_from: 0,
		span_to: 0,
		bar_line_type: BarLineType::Normal,
		y1: 0.0,
		y2: 0.0,
		elements: vec![]
	})}

	pub fn span_staff(&self) -> i32 { self.span_staff }
	pub fn set_span_staff(&mut self, v: i32) { self.span_staff = v }
	pub fn span_from(&self) -> i32 { self.span_from }
	pub fn set_span_from(&mut self, v: i32) { self.span_from = v }
	pub fn span_to(&self) -> i32 { self.span_to }
	pub fn set_span_to(&mut self, v: i32) { self.span_to = v }

	pub fn bar_line_type(&self) -> BarLineType { self.bar_line_type }
	pub fn set_bar_line_type(&mut self, v: BarLineType) { self.bar_line_type = v }

	pub fn y1(&self) -> f32 { self.y1 }
	pub fn set_y1(&mut self, v: f32) { self.y1 = v }
	pub fn y2(&self) -> f32 { self.y2 }
	pub fn set_y2(&mut self, v: f32) { self.y2 = v }
	pub fn elements(&self) -> &Vec<ElementRef> { &self.elements }
	pub fn set_elements(&mut self, v: Vec<ElementRef>) { self.elements = v }

	pub fn add(&mut self, e: ElementRef) {
		match e {
			ElementRef::Symbol(e) => self.elements.push(e.into()),
			ElementRef::Articulation(e) => self.elements.push(e.into()),
			_ => {},
		}
	}
	pub fn remove(&mut self, e: &ElementRef) {
		match e {
			ElementRef::Symbol(_) => remove_element(&mut self.elements, e),
			ElementRef::Articulation(_) => remove_element(&mut self.elements, e),
			_ => {},
		}
	}

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::BarlineType => ValueVariant::from_enum(self.bar_line_type()),
			PropertyId::BarlineSpan => self.span_staff().into(),
			PropertyId::BarlineSpanFrom => self.span_from().into(),
			PropertyId::BarlineSpanTo => self.span_to().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::BarlineType => v.with_enum(|v| self.set_bar_line_type(v)),
			PropertyId::BarlineSpan => v.with_value(|v| self.set_span_staff(v)),
			PropertyId::BarlineSpanFrom => v.with_value(|v| self.set_span_from(v)),
			PropertyId::BarlineSpanTo => v.with_value(|v| self.set_span_to(v)),
			_ => false,
		}
	}

}

impl Element for Barline {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Barline }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p).if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_custom_property(p, v.clone()) || self.set_element_property(p, v)
	}
}

impl SegmentTrait for Barline {
}

#[derive(Copy, Clone, Debug, Primitive, PartialEq, Eq)]
pub enum BarLineType {
	Normal = 1,
	Double = 2,
	StartRepeat = 4,
	EndRepeat = 8,
	Broken = 0x10,
	End = 0x20,
	EndStartRepeat = 0x40,
	Dotted = 0x80
}