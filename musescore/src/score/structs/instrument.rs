use super::*;

#[derive(Clone, Debug)]
pub struct InstrumentList {
	list: OrderedCollecton<Instrument>,
	default: Instrument,
}

impl InstrumentList {
	pub fn new(default: Instrument) -> Self { Self { list: OrderedCollecton::new(), default } }
	pub fn instrument(&self, tick: i32) -> &Instrument { self.list.current(tick).unwrap_or(&self.default) }
	pub fn set_instrument(&mut self, tick: i32, v: Instrument) { self.list.set_value(tick, v); }
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

	transpose: Interval,
	instrument_id: String,

	clef_type: Vec<ClefTypeList>,
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