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

	fn ticks(&self) -> &Fraction { &self.duration_data().duration }
	fn set_ticks(&mut self, v: Fraction) { self.duration_data_mut().duration = v }

	fn tuplet(&self) -> Option<El<Tuplet>> { self.duration_data().tuplet.as_ref().and_then(|e| e.upgrade()) }
	fn set_tuplet(&mut self, v: Option<ElWeak<Tuplet>>) { self.duration_data_mut().tuplet = v }

	fn get_duration_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Duration => ValueVariant::from_enum(self.ticks().ticks()),
			_ => ValueVariant::None
		}
	}
	fn set_duration_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Duration => v.with_enum(|v| self.duration_data_mut().duration = Fraction::from_ticks(v)),
			_ => false,
		}
	}
}