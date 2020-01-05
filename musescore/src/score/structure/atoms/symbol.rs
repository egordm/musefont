use crate::score::*;
use crate::font::*;

#[derive(Debug, Clone)]
pub struct Symbol {
	element: SymbolGroup,
	sym: SymName,
}

impl Symbol {
	pub fn default(score: Score) -> Self { Self {
		element: SymbolGroup::default(score),
		sym: SymName::NoSym,
	}}

	pub fn new(score: Score) -> El<Self> { new_element(Self::default(score))}

	pub fn sym(&self) -> SymName { self.sym }
	pub fn set_sym(&mut self, v: SymName) { self.sym = v }

	pub fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Symbol => ValueVariant::from_enum(self.sym()),
			_ => ValueVariant::None
		}
	}
	pub fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Symbol => v.with_enum(|v| self.set_sym(v)),
			_ => false,
		}
	}
}

impl Element for Symbol {
	fn el_data(&self) -> &ElementData { self.element.el_data() }
	fn el_data_mut(&mut self) -> &mut ElementData { self.element.el_data_mut() }

	fn element_type(&self) -> ElementType { ElementType::Symbol }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl AtomTrait for Symbol {

}
