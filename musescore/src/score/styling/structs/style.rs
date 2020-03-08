use super::*;
use crate::score::Spatium;

#[derive(Clone, Debug)]
pub struct Style {
	values: Vec<ValueVariant>,
	precomputed_values: Vec<f32>,
}

impl Style {
	pub fn new() -> Self {
		let mut values = Vec::new();
		values.resize_with(STYLE_COUNT, || ValueVariant::default());
		for (id, value) in style_default_values().iter() { values[*id as usize] = value.clone(); }
		Self { values, precomputed_values: vec![0.; STYLE_COUNT] }
	}

	pub fn precompute_values(&mut self) {
		let spatium = self.value(StyleName::Spatium).clone().flt();
		for (id, v) in self.values.iter().enumerate() {
			if let ValueVariant::Spatium(v) = v {
				self.precomputed_values[id] = v.points(spatium.into());
			}
		}
	}

	pub fn value(&self, id: impl Into<StyleId>) -> &ValueVariant { &self.values[id.into() as usize] }
	pub fn pvalue(&self, id: impl Into<StyleId>) -> f32 { self.precomputed_values[id.into() as usize] }
	pub fn set_value(&mut self, id: impl Into<StyleId>, v: ValueVariant) {
		let id = id.into();
		self.values[id as usize] = v.clone();
		if StyleName::Spatium == id { self.precompute_values(); }
		else if let ValueVariant::Spatium(v) = v {
			let spatium = self.value(StyleName::Spatium).clone().spt();
			self.precomputed_values[id as usize] = v.points(spatium.into());
		}
	}

	pub fn value_spatium(&self, id: impl Into<StyleId> + Copy) -> Spatium {
		if let ValueVariant::Spatium(v) = self.value(id) { *v }
		else { panic!("Style: {} has an incorrect value check style init", id.into()) }
	}
	pub fn value_p(&self, id: impl Into<StyleId> + Copy) -> f32 {
		if let ValueVariant::Spatium(_v) = self.value(id) { self.precomputed_values[id.into() as usize] }
		else { panic!("Style: {} has an incorrect value check style init", id.into()) }
	}
	pub fn value_bool(&self, id: impl Into<StyleId> + Copy) -> bool {
		if let ValueVariant::Bool(v) = self.value(id) { *v }
		else { panic!("Style: {} has an incorrect value check style init", id.into()) }
	}
	pub fn value_f32(&self, id: impl Into<StyleId> + Copy) -> f32 {
		if let ValueVariant::Float(v) = self.value(id) { *v }
		else { panic!("Style: {} has an incorrect value check style init", id.into()) }
	}
	pub fn value_i32(&self, id: impl Into<StyleId> + Copy) -> i32 {
		if let ValueVariant::Int(v) = self.value(id) { *v }
		else { panic!("Style: {} has an incorrect value check style init", id.into()) }
	}
	pub fn value_str(&self, id: impl Into<StyleId> + Copy) -> String {
		if let ValueVariant::String(v) = self.value(id) { v.clone() }
		else { panic!("Style: {} has an incorrect value check style init", id.into()) }
	}

	// TODO: loading
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_type() {
		let style = Style::new();

		dbg!(style.value(StyleName::Spatium));
		assert_eq!(style.value(StyleName::Spatium).clone().flt(), 25.);
	}
}