use crate::score::*;

#[derive(Clone, Debug)]
pub struct StaffType {
	group: StaffGroup,

	user_mag: f32,
	voffset: Spatium,
	small: bool,
	lines: Line,
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
	notehead_scheme: NoteheadScheme,

	// TODO: tabbed layout params
}

impl StaffType {
	pub fn group(&self) -> &StaffGroup { &self.group }
	pub fn set_group(&mut self, v: StaffGroup) { self.group = v }

	pub fn user_mag(&self) -> f32 { self.user_mag }
	pub fn set_user_mag(&mut self, v: f32) { self.user_mag = v }

	pub fn voffset(&self) -> Spatium { self.voffset }
	pub fn set_voffset(&mut self, v: Spatium) { self.voffset = v }

	pub fn small(&self) -> bool { self.small }
	pub fn set_small(&mut self, v: bool) { self.small = v }

	pub fn lines(&self) -> Line { self.lines }
	pub fn set_lines(&mut self, v: Line) { self.lines = v }

	pub fn step_offset(&self) -> i32 { self.step_offset }
	pub fn set_step_offset(&mut self, v: i32) { self.step_offset = v }

	pub fn line_distance(&self) -> Spatium { self.line_distance }
	pub fn set_line_distance(&mut self, v: Spatium) { self.line_distance = v }
	pub fn show_bar_lines(&self) -> bool { self.show_bar_lines }
	pub fn set_show_bar_lines(&mut self, v: bool) { self.show_bar_lines = v }
	pub fn show_ledger_lines(&self) -> bool { self.show_ledger_lines }
	pub fn set_show_ledger_lines(&mut self, v: bool) { self.show_ledger_lines = v }

	pub fn stemless(&self) -> bool { self.stemless }
	pub fn set_stemless(&mut self, v: bool) { self.stemless = v }

	pub fn gen_clef(&self) -> bool { self.gen_clef }
	pub fn set_gen_clef(&mut self, v: bool) { self.gen_clef = v }
	pub fn gen_timesig(&self) -> bool { self.gen_timesig }
	pub fn set_gen_timesig(&mut self, v: bool) { self.gen_timesig = v }

	pub fn gen_keysig(&self) -> bool { self.gen_keysig }
	pub fn set_gen_keysig(&mut self, v: bool) { self.gen_keysig = v }
	pub fn notehead_scheme(&self) -> NoteheadScheme { self.notehead_scheme }
	pub fn set_notehead_scheme(&mut self, v: NoteheadScheme) { self.notehead_scheme = v }
}

impl Default for StaffType {
	fn default() -> Self { Self {
		group: StaffGroup::Standard,
		user_mag: 1.0,
		voffset: Spatium(0.0),
		small: false,
		lines: Line::from(5),
		step_offset: 0,
		line_distance: Spatium(1.0),
		show_bar_lines: true,
		show_ledger_lines: true,
		stemless: false,
		gen_clef: true,
		gen_timesig: true,
		gen_keysig: true,
		notehead_scheme: NoteheadScheme::Normal,
	}}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StaffGroup {
	Standard,
	Percussion,
	Tab
}