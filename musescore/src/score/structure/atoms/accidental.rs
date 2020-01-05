use crate::font::SymName;
use crate::score::*;

/// # Accidental
#[derive(Debug, Clone)]
pub struct Accidental {
	element: ElementData,

	elements: Vec<SymElement>,
	accidental_type: AccidentalType,
	small: bool,
	bracket: AccidentalBracket,
	role: AccidentalRole,
}

impl Accidental {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		elements: vec![],
		accidental_type: AccidentalType::None,
		small: false,
		bracket: AccidentalBracket::None,
		role: AccidentalRole::Auto
	})}

	pub fn elements(&self) -> &Vec<SymElement> { &self.elements }
	pub fn set_elements(&mut self, v: Vec<SymElement>) { self.elements = v }

	pub fn accidental_type(&self) -> AccidentalType { self.accidental_type }
	pub fn set_accidental_type(&mut self, v: AccidentalType) { self.accidental_type = v }

	pub fn small(&self) -> bool { self.small }
	pub fn set_small(&mut self, v: bool) { self.small = v }

	pub fn bracket(&self) -> AccidentalBracket { self.bracket }
	pub fn set_bracket(&mut self, v: AccidentalBracket) { self.bracket = v }

	pub fn role(&self) -> AccidentalRole { self.role }
	pub fn set_role(&mut self, v: AccidentalRole) { self.role = v }

	pub fn symbol(&self) -> SymName { ACC_LIST[self.accidental_type as usize].2 }
	pub fn note(&self) -> Option<El<Note>> { self.parent_ty() }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::AccidentalType => ValueVariant::from_enum(self.accidental_type()),
			PropertyId::Small => self.small().into(),
			PropertyId::AccidentalBracket => ValueVariant::from_enum(self.bracket()),
			PropertyId::Role => ValueVariant::from_enum(self.role()),
			_ => ValueVariant::None,
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::AccidentalType => v.with_enum(|v| self.set_accidental_type(v)),
			PropertyId::Small => v.with_value(|v| self.set_small(v)),
			PropertyId::AccidentalBracket => v.with_enum(|v| self.set_bracket(v)),
			PropertyId::Role => v.with_enum(|v| self.set_role(v)),
			_ => false,
		}
	}
}

impl Element for Accidental {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Accidental }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl AtomTrait for Accidental {

}

#[derive(Debug, Clone)]
pub struct SymElement {
	sym: SymName,
	x: f32,
}

impl SymElement {
	pub fn new(sym: SymName, x: f32) -> Self {Self{sym, x}}
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum AccidentalBracket {
	None = 0,
	Parenthesis = 1,
	Bracket = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Primitive, Eq, Hash)]
pub enum AccidentalRole {
	/// layout created accidental
	Auto = 0,
	/// user created accidental
	User = 1
}