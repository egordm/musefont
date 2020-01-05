use crate::score::*;

/// Represents a Key Signature on a staff
#[derive(Debug, Clone)]
pub struct KeySig {
	element: ElementData,

	/// Show courtesy key signature for this sig if appropriate
	show_courtesy: bool,
	/// Used in layout to override score style (needed for the Continuous panel)
	hide_naturals: bool,
	sig: KeySigEvent,
}

impl Element for KeySig {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::KeySig }
}

impl SegmentTrait for KeySig {
}