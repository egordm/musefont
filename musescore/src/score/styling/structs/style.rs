use crate::*;
use super::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::TryInto;

pub type Spatium = f32;
pub type Color = [u8; 4];

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
		let spatium = self.value(StyleName::Spatium).spatium();
		for (id, v) in self.values.iter().enumerate() {
			if let ValueVariant::Spatium(v) = v {
				self.precomputed_values[id] = v * spatium;
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
			let spatium = self.value(StyleName::Spatium).spatium();
			self.precomputed_values[id as usize] = v * spatium;
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

#[derive(Clone, Debug, PartialEq)]
pub enum ValueVariant {
	None,
	Spatium(Spatium),
	Bool(bool),
	Float(f32),
	Int(i32),
	UInt(u32),
	Point(Point2F),
	String(String),
	Color(Color)
}

impl ValueVariant {
	pub fn with_value<T, F: FnMut(T)>(self, mut f: F) -> bool where ValueVariant: TryInto<T> {
		if let Ok(v) = self.try_into() { f(v); true } else { false }
	}
	pub fn try_into_enum<T: FromPrimitive>(self) -> Result<T, ()> {
		if let ValueVariant::UInt(v) = self {
			if let Some(v) = T::from_u32(v) { return Ok(v); }
		}
		if let ValueVariant::Int(v) = self {
			if let Some(v) = T::from_i32(v) { return Ok(v); }
		}
		Err(())
	}
	pub fn with_enum<T: FromPrimitive, F: FnMut(T)>(self, mut f: F) -> bool {
		if let Ok(v) = self.try_into_enum() { f(v); true } else { false }
	}
	pub fn from_enum<T: ToPrimitive>(v: T) -> Self {
		if let Some(v) = v.to_u32() { ValueVariant::UInt(v) }
		else if let Some(v) = v.to_i32() { ValueVariant::Int(v) }
		else { ValueVariant::None }
	}

	pub fn try_into_spatium(self) -> Result<Spatium, ()> {
		if let ValueVariant::Spatium(v) = self { Ok(v) } else { Err(()) }
	}
	pub fn with_spatium<F: FnMut(f32)>(self, mut f: F) -> bool {
		if let Ok(v) = self.try_into_spatium() { f(v); true } else { false }
	}
	pub fn spatium(&self) -> Spatium {
		if let ValueVariant::Spatium(s) = self { *s } else { Default::default() }
	}

	pub fn if_none<F: FnMut() -> ValueVariant>(self, mut f: F) -> ValueVariant {
		if self == ValueVariant::None { f() } else { self }
	}
}

impl TryInto<bool> for ValueVariant {
	type Error = ();
	fn try_into(self) -> Result<bool, Self::Error> {
		if let ValueVariant::Bool(v) = self { Ok(v) } else { Err(()) }
	}
}
impl From<bool> for ValueVariant {
	fn from(v: bool) -> Self { ValueVariant::Bool(v) }
}
impl TryInto<f32> for ValueVariant {
	type Error = ();
	fn try_into(self) -> Result<f32, Self::Error> {
		if let ValueVariant::Float(v) = self { Ok(v) } else { Err(()) }
	}
}
impl From<f32> for ValueVariant {
	fn from(v: f32) -> Self { ValueVariant::Float(v) }
}
impl TryInto<i32> for ValueVariant {
	type Error = ();
	fn try_into(self) -> Result<i32, Self::Error> {
		if let ValueVariant::Int(v) = self { Ok(v) } else { Err(()) }
	}
}
impl From<i32> for ValueVariant {
	fn from(v: i32) -> Self { ValueVariant::Int(v) }
}
impl TryInto<u32> for ValueVariant {
	type Error = ();
	fn try_into(self) -> Result<u32, Self::Error> {
		if let ValueVariant::UInt(v) = self { Ok(v) } else { Err(()) }
	}
}
impl From<u32> for ValueVariant {
	fn from(v: u32) -> Self { ValueVariant::UInt(v) }
}
impl TryInto<Point2F> for ValueVariant {
	type Error = ();
	fn try_into(self) -> Result<Point2F, Self::Error> {
		if let ValueVariant::Point(v) = self { Ok(v) } else { Err(()) }
	}
}
impl From<&Point2F> for ValueVariant {
	fn from(v: &Point2F) -> Self { ValueVariant::Point(v.clone()) }
}
impl From<Point2F> for ValueVariant {
	fn from(v: Point2F) -> Self { ValueVariant::Point(v) }
}
impl TryInto<String> for ValueVariant {
	type Error = ();
	fn try_into(self) -> Result<String, Self::Error> {
		if let ValueVariant::String(v) = self { Ok(v) } else { Err(()) }
	}
}
impl From<String> for ValueVariant {
	fn from(v: String) -> Self { ValueVariant::String(v) }
}
impl TryInto<Color> for ValueVariant {
	type Error = ();
	fn try_into(self) -> Result<Color, Self::Error> {
		if let ValueVariant::Color(v) = self { Ok(v) } else { Err(()) }
	}
}
impl From<Color> for ValueVariant {
	fn from(v: Color) -> Self { ValueVariant::Color(v) }
}

impl Default for ValueVariant {
	fn default() -> Self { ValueVariant::None }
}
