use crate::score::*;
use std::convert::TryInto;

pub trait MeasureTrait: Element {
	fn measure_data(&self) -> &MeasureData;
	fn measure_data_mut(&mut self) -> &mut MeasureData;

	fn system(&self) -> Option<El<System>> { self.parent().and_then(|e| e.try_into().ok()) }
}

#[derive(Debug, Clone)]
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