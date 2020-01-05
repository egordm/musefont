use crate::font::SymName;
use crate::score::*;

/// # Articulation
/// articulation marks
#[derive(Debug, Clone)]
pub struct Articulation {
	element: ElementData,

	sym_id: SymName,
	direction: DirectionV,
	channel_name: String,

	anchor: ArticulationAnchor,

	up: bool,
	/// for use in ornaments such as trill
	ornament_style: OrnamentStyle,
}

impl Articulation {
	pub fn new(score: Score) -> El<Self> {
		new_element(Self {
			element: ElementData::new(score),
			sym_id: SymName::NoSym,
			direction: DirectionV::Auto,
			channel_name: String::new(),
			anchor: ArticulationAnchor::TopStaff,
			up: true,
			ornament_style: OrnamentStyle::Default
		})
	}

	pub fn sym(&self) -> SymName { self.sym_id }
	pub fn set_sym(&mut self, v: SymName) { self.sym_id = v }

	pub fn direction(&self) -> DirectionV { self.direction }
	pub fn set_direction(&mut self, v: DirectionV) { self.direction = v }

	pub fn channel_name(&self) -> &String { &self.channel_name }
	pub fn set_channel_name(&mut self, v: String) { self.channel_name = v }

	pub fn anchor(&self) -> ArticulationAnchor { self.anchor }
	pub fn set_anchor(&mut self, v: ArticulationAnchor) { self.anchor = v }

	pub fn up(&self) -> bool { self.up }
	pub fn set_up(&mut self, v: bool) { self.up = v }

	pub fn ornament_style(&self) -> OrnamentStyle { self.ornament_style }
	pub fn set_ornament_style(&mut self, v: OrnamentStyle) { self.ornament_style = v }

	// Types
	pub fn layout_close_to_note(&self) -> bool {
		(self.is_staccato() || self.is_tenuto()) && !self.is_double()
	}

	pub fn is_tenuto(&self) -> bool {
		match self.sym_id {
			SymName::ArticTenutoAbove | SymName::ArticTenutoBelow => true,
			_ => false
		}
	}
	pub fn is_staccato(&self) -> bool {
		match self.sym_id {
			SymName::ArticStaccatoAbove | SymName::ArticStaccatoBelow | SymName::ArticMarcatoStaccatoAbove |
			SymName::ArticMarcatoStaccatoBelow | SymName::ArticAccentStaccatoAbove | SymName::ArticAccentStaccatoBelow => true,
			_ => false
		}
	}
	pub fn is_accent(&self) -> bool {
		match self.sym_id {
			SymName::ArticAccentAbove | SymName::ArticAccentBelow |
			SymName::ArticAccentStaccatoAbove | SymName::ArticAccentStaccatoBelow => true,
			_ => false
		}
	}
	pub fn is_marcato(&self) -> bool {
		match self.sym_id {
			SymName::ArticMarcatoAbove | SymName::ArticMarcatoBelow | SymName::ArticMarcatoStaccatoAbove |
			SymName::ArticMarcatoStaccatoBelow | SymName::ArticMarcatoTenutoAbove | SymName::ArticMarcatoTenutoBelow => true,
			_ => false
		}
	}
	pub fn is_double(&self) -> bool {
		match self.sym_id {
			SymName::ArticMarcatoStaccatoAbove | SymName::ArticMarcatoStaccatoBelow | SymName::ArticAccentStaccatoAbove |
			SymName::ArticAccentStaccatoBelow | SymName::ArticMarcatoTenutoAbove | SymName::ArticMarcatoTenutoBelow => true,
			_ => false
		}
	}
	pub fn is_ornament(&self) -> bool {
		match self.sym_id {
			SymName::OrnamentTurn | SymName::OrnamentTurnInverted | SymName::OrnamentTrill | SymName::BrassMuteClosed |
			SymName::OrnamentMordentInverted | SymName::OrnamentMordent | SymName::OrnamentTremblement |
			SymName::OrnamentPrallMordent | SymName::OrnamentLinePrall | SymName::OrnamentUpPrall |
			SymName::OrnamentUpMordent | SymName::OrnamentPrecompMordentUpperPrefix | SymName::OrnamentDownMordent |
			SymName::OrnamentPrallUp | SymName::OrnamentPrallDown | SymName::OrnamentPrecompSlide => true,
			_ => false
		}
	}

	// Property setting
	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Symbol => ValueVariant::from_enum(self.sym()),
			PropertyId::Direction => ValueVariant::from_enum(self.direction()),
			PropertyId::ArticulationAnchor => ValueVariant::from_enum(self.anchor()),
			PropertyId::OrnamentStyle => ValueVariant::from_enum(self.ornament_style()),
			_ => ValueVariant::None,
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Symbol => v.with_enum(|v| self.set_sym(v)),
			PropertyId::Direction => v.with_enum(|v| self.set_direction(v)),
			PropertyId::ArticulationAnchor => v.with_enum(|v| self.set_anchor(v)),
			PropertyId::OrnamentStyle => v.with_enum(|v| self.set_ornament_style(v)),
			_ => false,
		}
	}
}

impl Element for Articulation {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Articulation }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl AtomTrait for Articulation {

}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum ArticulationAnchor {
	/// anchor is always placed at top of staff
	TopStaff = 0,
	/// anchor is always placed at bottom of staff
	BottomStaff = 1,
	/// anchor depends on chord direction, away from stem
	Chord = 2,
	/// attribute is always placed at top of chord
	TopChord = 3,
	/// attribute is placed at bottom of chord
	BottomChord = 4,
}