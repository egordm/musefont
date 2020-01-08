use crate::*;
use crate::score::*;
use crate::font::SymName;

#[derive(Debug, Clone)]
pub struct Rest {
	element: ElementData,
	duration_data: DurationElementData,
	rest_data: ChordRestData,

	sym: SymName,
	/// depends on rest symbol
	dotline: i32,
	/// width of multi measure rest
	mm_width: f32,
	/// invisible and not selectable for user
	gap: bool,
	dots: Vec<El<NoteDot>>
}

impl Rest {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		duration_data: DurationElementData::new(Fraction::default()),
		rest_data: ChordRestData::default(),

		sym: SymName::NoSym,
		dotline: -1,
		mm_width: 0.0,
		gap: false,
		dots: vec![]
	})}

	pub fn sym(&self) -> &SymName { &self.sym }
	pub fn dotline(&self) -> i32 { self.dotline }
	pub fn mm_width(&self) -> f32 { self.mm_width }

	pub fn gap(&self) -> bool { self.gap }
	pub fn set_gap(&mut self, v: bool) { self.gap = v }

	pub fn up_line(&self) -> i32 {
		let spatium = self.spatium();
		((self.pos().y + self.bbox().top() + spatium) * 2.0 / spatium).round() as i32
	}
	pub fn down_line(&self) -> i32 {
		let spatium = self.spatium();
		((self.pos().y + self.bbox().top() + spatium) * 2.0 / spatium).round() as i32
	}
	/// point to connect stem
	pub fn stem_pos(&self) -> Point2F {
		Point2F::default() // TODO: stem pos
	}
	pub fn stem_posx(&self) -> f32 {
		if self.up() { self.bbox().right() } else { self.bbox().left() }
	}
	/// return stem position of note on beam side
	/// return canvas coordinates
	pub fn stem_pos_beam(&self) -> Point2F {
		let mut p = Point2F::default(); // TODO: page pos
		if self.up() { p.y += self.bbox().top() + self.spatium() * 1.5; }
		else { p.y += self.bbox().bottom() + self.spatium() * 1.5 }
		p
	}

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Gap => self.gap().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Gap => v.with_value(|v| self.set_gap(v)),
			_ => false,
		}
	}
}

impl Element for Rest {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Rest }

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

impl DurationElement for Rest {
	fn duration_data(&self) -> &DurationElementData { &self.duration_data }
	fn duration_data_mut(&mut self) -> &mut DurationElementData { &mut self.duration_data }
}

impl ChordRestTrait for Rest {
	fn rest_data(&self) -> &ChordRestData { &self.rest_data }
	fn rest_data_mut(&mut self) -> &mut ChordRestData { &mut self.rest_data }
}

impl SegmentTrait for Rest {
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CrossMeasure {
	Unknown = -1,
	None = 0,
	First = 1,
	Second = 2
}
