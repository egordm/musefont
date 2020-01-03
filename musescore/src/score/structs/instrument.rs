use super::*;

#[derive(Clone, Debug)]
pub struct Instrument {
	long_names: Vec<StaffName>,
	short_names: Vec<StaffName>,
	track_name: String,

	min_pitch_a: u8,
	max_pitch_a: u8,
	min_pitch_p: u8,
	max_pitch_p: u8,

	transpose: Interval,
	instrument_id: String,

	clef_type: Vec<ClefTypeGroup>,
}

#[derive(Clone, Debug)]
pub struct StaffName {
	name: String,
	pos: i32, // even number -> between staves
}

impl StaffName {
	pub fn new(name: String, pos: i32) -> Self { Self { name, pos } }
	pub fn name(&self) -> &str  { &self.name }
	pub fn pos(&self) -> i32  { self.pos }
}