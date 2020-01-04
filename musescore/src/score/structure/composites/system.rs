use crate::score::*;

/// One row of measures for all instruments;
/// a complete piece of the timeline
#[derive(Debug, Clone)]
pub struct System {
	element: ElementData,

	// TODO
}

impl Element for System {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::System }
}
