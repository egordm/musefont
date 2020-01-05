use crate::ClefTypeList;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct InstrumentList {
	list: BTreeMap<i32, Instrument>,
	default: Instrument,
}

impl InstrumentList {
	pub fn new(default: Instrument) -> Self { Self { list: BTreeMap::new(), default } }
	pub fn instrument(&self, tick: i32) -> &Instrument {
		if let Some((_, v)) = self.list.range(tick + 1..).next_back() { v }
		else { &self.default }
	}
	pub fn set_instrument(&mut self, tick: i32, v: Instrument) {
		if let Some(current) = self.list.get_mut(&tick) { *current = v; }
		else { self.list.insert(tick, v); }
	}
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

#[derive(Clone, Debug)]
pub struct Instrument {
	long_names: Vec<StaffName>,
	short_names: Vec<StaffName>,
	track_name: String,

	min_pitch_a: u8,
	max_pitch_a: u8,
	min_pitch_p: u8,
	max_pitch_p: u8,

	// transpose: Interval,
	instrument_id: String,

	clef_type: Vec<ClefTypeList>,
}