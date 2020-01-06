use crate::score::*;

/// # Spacer
/// Vertical spacer element to adjust the distance of staves.
#[derive(Debug, Clone)]
pub struct Spacer {
	element: ElementData,

	spacer_type: SpacerType,
	gap: f32,
	//path: PolyLine,
}

impl Spacer {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		spacer_type: SpacerType::Up,
		gap: 0.0
	})}

	pub fn spacer_type(&self) -> SpacerType { self.spacer_type }
	pub fn set_spacer_type(&mut self, v: SpacerType) { self.spacer_type = v }
	pub fn gap(&self) -> f32 { self.gap }
	pub fn set_gap(&mut self, v: f32) { self.gap = v }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Space => self.gap().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Space => v.with_value(|v| self.set_gap(v)),
			_ => false,
		}
	}


}

impl Element for Spacer {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Chordline }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl AtomTrait for Spacer {

}


#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum SpacerType {
	Up = 0,
	Down = 1,
	Fixed = 2,
}