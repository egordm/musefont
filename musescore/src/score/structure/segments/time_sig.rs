use crate::*;
use crate::score::*;
use crate::font::SymName;

/// This class represents a time signature.
#[derive(Debug, Clone)]
pub struct TimeSig {
	element: ElementData,
	segment_data: SegmentData,

	/// calculated from actualSig() if !customText
	numerator_string: String,
	denumerator_string: String,

	ns: Vec<SymName>,
	ds: Vec<SymName>,

	pz: Point2F,
	pn: Point2F,

	point_large_left_paren: Point2F,
	point_large_right_paren: Point2F,

	sig: Fraction,
	/// localSig / globalSig
	stretch: Fraction,
	//groups: Groups,

	scale: Size2F,
	time_sig_type: TimeSigType,
	show_courtesy_sig: bool,
	large_parentheses: bool,
}

impl Element for TimeSig {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}

impl SegmentTrait for TimeSig {
	fn segment_data(&self) -> &SegmentData { &self.segment_data }
	fn segment_data_mut(&mut self) -> &mut SegmentData { &mut self.segment_data }
}

#[derive(Clone, Copy, Debug)]
pub enum TimeSigType {
	/// use sz/sn text
	Normal,
	/// common time (4/4)
	FourFour,
	/// cut time (2/2)
	AllaBreve,
}