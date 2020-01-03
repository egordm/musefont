use crate::score::*;

#[derive(Debug, Clone)]
pub struct DurationElementData {
	duration: Fraction,
	tuplet: Option<ElWeak<Tuplet>>
}

impl DurationElementData {
	pub fn new(duration: Fraction) -> Self { Self { duration, tuplet: None }}
}

pub trait DurationElement: Element {
	fn duration_data(&self) -> &DurationElementData;
	fn duration_data_mut(&mut self) -> &mut DurationElementData;
}