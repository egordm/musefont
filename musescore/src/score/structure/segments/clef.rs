use crate::score::*;
use crate::font::SymName;

#[derive(Debug, Clone)]
pub struct Clef {
	element: ElementData,

	sym_id: SymName,
	show_courtesy: bool,
	small: bool,

	clef_types: ClefTypeGroup,
}

impl Clef {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		sym_id: SymName::NoSym,
		show_courtesy: true,
		small: false,
		clef_types: ClefTypeGroup::from_clef(ClefType::Invalid),
	})}

	pub fn sym_id(&self) -> SymName { self.sym_id }
	pub fn set_sym_id(&mut self, v: SymName) { self.sym_id = v }

	pub fn show_courtesy(&self) -> bool { self.show_courtesy }
	pub fn set_show_courtesy(&mut self, v: bool) { self.show_courtesy = v }
	pub fn small(&self) -> bool { self.small }
	pub fn set_small(&mut self, v: bool) { self.small = v }

	pub fn clef_type(&self) -> ClefType {
		if self.concert_pitch() { self.clef_types.concert_clef() }
		else { self.clef_types.transposing_clef() }
	}

	pub fn set_concert_clef(&mut self, v: ClefType) { self.clef_types.set_concert_clef(v) }
	pub fn set_transposing_clef(&mut self, v: ClefType) { self.clef_types.set_transposing_clef(v) }

	pub fn clef_type_group(&self) -> &ClefTypeGroup { &self.clef_types }
	pub fn set_clef_type_group(&mut self, v: ClefTypeGroup) { self.clef_types = v }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::ClefTypeConcert => ValueVariant::from_enum(self.clef_type_group().concert_clef()),
			PropertyId::ClefTypeTransposing => ValueVariant::from_enum(self.clef_type_group().transposing_clef()),
			PropertyId::ShowCourtesy => self.show_courtesy().into(),
			PropertyId::Small => self.small().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::ClefTypeConcert => v.with_enum(|v| self.set_concert_clef(v)),
			PropertyId::ClefTypeTransposing => v.with_enum(|v| self.set_transposing_clef(v)),
			PropertyId::ShowCourtesy => v.with_value(|v| self.set_show_courtesy(v)),
			PropertyId::Small => v.with_value(|v| self.set_small(v)),
			_ => false,
		}
	}
}

impl Element for Clef {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Clef }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p).if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl SegmentTrait for Clef {
}