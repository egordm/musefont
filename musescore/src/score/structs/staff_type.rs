use crate::score::*;

#[derive(Clone, Debug)]
pub struct StaffType {
	group: StaffGroup,

	user_mag: f32,
	voffset: Spatium,
	small: bool,
	lines: u32,
	step_offset: i32,
	line_distance: Spatium,

	show_bar_lines: bool,
	show_ledger_lines: bool,
	stemless: bool,

	/// create clef at beginning of system
	gen_clef: bool,
	/// whether time signature is shown or not
	gen_timesig: bool,
	/// create key signature at beginning of system
	gen_keysig: bool,

	// Standard: configurable properties
	//note_head_scheme: notehead::Scheme,

	// TODO: tabbed layout params
}

impl Default for StaffType {
	fn default() -> Self { Self {
		group: StaffGroup::Standard,
		user_mag: 1.0,
		voffset: 0.0,
		small: false,
		lines: 5,
		step_offset: 0,
		line_distance: 1.0,
		show_bar_lines: true,
		show_ledger_lines: true,
		stemless: false,
		gen_clef: true,
		gen_timesig: true,
		gen_keysig: true,
		//note_head_scheme: notehead::Scheme::Normal,
	}}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StaffGroup {
	Standard,
	Percussion,
	Tab
}