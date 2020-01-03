use crate::score::*;

pub trait MeasureTrait {

}

pub struct MeasureData {
	elements: Vec<ElementRef>,
	tick: Fraction,
	len: Fraction,
	number: i32,
	number_offser: i32,
}

impl Default for MeasureData {
	fn default() -> Self { Self {
		elements: vec![],
		tick: Fraction::new(0, 1),
		len: Fraction::new(0, 1),
		number: 0,
		number_offser: 0
	}}
}