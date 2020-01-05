use crate::*;

#[derive(Clone, Debug)]
pub struct Measure {
	element: MeasureBase,

	timesig: Fraction,
	repeat_count: i32,
}

impl MeasureTrait for Measure {
	fn measure(&self) -> &MeasureBase { &self.element }
	fn measure_mut(&mut self) -> &mut MeasureBase { &mut self.element }
}

impl ElementTrait for Measure {
	fn el(&self) -> &Element { self.element.el() }
	fn el_mut(&mut self) -> &mut Element { self.element.el_mut() }
	fn element_type(&self) -> ElementType { ElementType::Measure }
}
