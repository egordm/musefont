use crate::structs::style_id::style_default_values;
use crate::*;

pub type Spatium = f32;
pub type Color = [u8; 4];

#[derive(Clone, Debug)]
pub enum StyleValue {
	None,
	Spatium(Spatium),
	Bool(bool),
	Float(f32),
	Int(i32),
	UInt(u32),
	Point(Point2F),
	String(String),
	Color(Color),
}

impl Default for StyleValue {
	fn default() -> Self { StyleValue::None }
}

impl StyleValue {
	pub fn spatium(&self) -> Spatium {
		if let StyleValue::Spatium(s) = self { *s } else { Default::default() }
	}
}

//pub static  DEFAULT_VALUES: [(StyleValue); 1] = [StyleValue::String(String::from("aa"))];
pub const A: Color = [1, 2, 3, 4];

#[derive(Clone, Debug)]
pub struct Style {
	values: Vec<StyleValue>,
	precomputed_values: Vec<f32>,
}

impl Style {
	pub fn new() -> Self {
		let mut values = Vec::new();
		values.resize_with(STYLE_COUNT, || StyleValue::default());
		for (id, value) in style_default_values().iter() { values[*id as usize] = value.clone(); }

		Self { values, precomputed_values: vec![0.; STYLE_COUNT] }
	}

	pub fn precompute_values(&mut self) {
		let spatium = self.value(StyleId::Spatium as SId).spatium();
		for (id, v) in self.values.iter().enumerate() {
			if let StyleValue::Spatium(v) = v {
				self.precomputed_values[id] = v * spatium;
			}
		}
	}

	pub fn value(&self, id: SId) -> &StyleValue { &self.values[id as usize] }
	pub fn pvalue(&self, id: SId) -> f32 { self.precomputed_values[id as usize] }
	pub fn set_value(&mut self, id: SId, v: StyleValue) {
		self.values[id as usize] = v.clone();
		if StyleId::Spatium == id { self.precompute_values(); }
		else if let StyleValue::Spatium(v) = v {
			let spatium = self.value(StyleId::Spatium as SId).spatium();
			self.precomputed_values[id as usize] = v * spatium;
		}
	}

	pub fn value_spatium(&self, id: SId) -> Spatium {
		if let StyleValue::Spatium(v) = self.value(id) { *v }
		else { panic!("Style: {} has an incorrect value check style init", id) }
	}
	pub fn value_p(&self, id: SId) -> f32 {
		if let StyleValue::Spatium(v) = self.value(id) { self.precomputed_values[id as usize] }
		else { panic!("Style: {} has an incorrect value check style init", id) }
	}
	pub fn value_bool(&self, id: SId) -> bool {
		if let StyleValue::Bool(v) = self.value(id) { *v }
		else { panic!("Style: {} has an incorrect value check style init", id) }
	}
	pub fn value_f32(&self, id: SId) -> f32 {
		if let StyleValue::Float(v) = self.value(id) { *v }
		else { panic!("Style: {} has an incorrect value check style init", id) }
	}
	pub fn value_i32(&self, id: SId) -> i32 {
		if let StyleValue::Int(v) = self.value(id) { *v }
		else { panic!("Style: {} has an incorrect value check style init", id) }
	}
	pub fn value_str(&self, id: SId) -> String {
		if let StyleValue::String(v) = self.value(id) { v.clone() }
		else { panic!("Style: {} has an incorrect value check style init", id) }
	}

	// TODO: loading
}