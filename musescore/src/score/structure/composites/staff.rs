use crate::score::*;

/// Global staff data not directly related to drawing.
#[derive(Debug, Clone)]
pub struct Staff {
	element: ElementData,

	/// Part the staff element belongs to
	part: Option<ElWeak<Part>>,

	/// List of Clefs indexed using Ticks
	clefs: ClefTypeList,
	default_clef_type: ClefTypeGroup,

	/// List of Keys indexed using Ticks
	keys: KeyList,
	/// List of TimeSignatures indexed using Ticks
	timesigs: TimesigList,

	/// span barline to next staff
	bar_line_span: i32,
	/// line of start staff to draw the barline from (0 = staff top line, ...)
	bar_line_from: i32,
	/// line of end staff to draw the bar line to (0= staff bottom line, ...)
	bar_line_to: i32,

	color: Color,

	/// List of Staff Types indexed using Ticks
	staff_type_list: StaffTypeList
}

impl Element for Staff {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}

pub type ClefTypeList = OrderedCollecton<ClefTypeGroup>;
pub type KeyList = OrderedCollecton<KeySigEvent>;
pub type TimesigList = OrderedCollecton<El<TimeSig>>;
pub type StaffTypeList = OrderedCollecton<StaffType>;