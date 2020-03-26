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

	fn global_duration(&self) -> Fraction {
		let mut f = self.duration_data().duration;
		let mut t = self.tuplet();
		while let Some(tuplet) = t {
			f /= *tuplet.borrow_el().ratio();
			t = tuplet.borrow_el().tuplet();
		}
		return f;
	}
	fn actual_duration(&self) -> Fraction {
		let stretch_default = Fraction::new(1, 1);
		self.global_duration() / self.staff().with_d(|d| d.timestretch(&self.time()), stretch_default)
	}

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