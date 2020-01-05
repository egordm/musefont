use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct ClefList(BTreeMap<i32, ClefTypeList>);

impl ClefList {
	pub fn new() -> Self { Self(BTreeMap::new()) }

	pub fn clef(&self, tick: i32) -> ClefTypeList {
		if let Some((_, v)) = self.0.range(tick + 1..).next_back() { v.clone() }
		else { ClefTypeList::new(ClefType::Invalid, ClefType::Invalid) }
	}
	pub fn set_clef(&mut self, tick: i32, v: ClefTypeList) {
		if let Some(current) = self.0.get_mut(&tick) { *current = v; }
		else { self.0.insert(tick, v); }
	}
	/// return the tick at which the clef after tick is located
	/// return -1, if no such clef
	pub fn next_clef_tick(&self, tick: i32) -> i32 {
		if let Some((k, _)) = self.0.range(tick + 2..).next() { *k }
		else { -1 }
	}
	/// return the tick position of the clef currently in effect at tick
	pub fn current_clef_tick(&self, tick: i32) -> i32 {
		if let Some((k, _)) = self.0.range(tick + 1..).next_back() { *k }
		else { 0 }
	}
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClefTypeList {
	concert_clef: ClefType,
	transposing_clef: ClefType
}

impl ClefTypeList {
	pub fn new(concert_clef: ClefType, transposing_clef: ClefType) -> Self { Self { concert_clef, transposing_clef}}

	pub fn from_clef(concert_clef: ClefType) -> Self { Self::new(concert_clef, concert_clef) }
}

impl Default for ClefTypeList {
	fn default() -> Self { Self::new(ClefType::G, ClefType::G) }
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum ClefType {
	Invalid = 0,
	G = 1,
	G15Mb = 2,
	G8Vb = 3,
	G8Va = 4,
	G15Ma = 5,
	G8VbO = 6,
	G8VbP = 7,
	G1 = 8,
	C1 = 9,
	C2 = 10,
	C3 = 11,
	C4 = 12,
	C5 = 13,
	C19C = 14,
	C3F18C = 15,
	C4F18C = 16,
	C3F20C = 17,
	C4F20C = 18,
	F = 19,
	F15Mb = 20,
	F8Vb = 21,
	F8Va = 22,
	F15Ma = 23,
	FB = 24,
	FC = 25,
	FF18C = 26,
	F19C = 27,
	Perc = 28,
	Perc2 = 29,
	Tab = 30,
	Tab4 = 31,
	TabSerif = 32,
	Tab4Serif = 33,
	Max = 34,
}