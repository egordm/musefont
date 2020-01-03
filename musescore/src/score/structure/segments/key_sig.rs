use crate::score::*;

#[derive(Debug, Clone)]
pub struct KeySig {
	show_courtesy: bool,
	/// used in layout to override score style (needed for the Continuous panel)
	hide_naturals: bool,
	sig: KeySigEvent,
}

impl SegmentTrait for KeySig {

}