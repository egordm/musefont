#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClefTypeGroup {
	concert_clef: ClefType,
	transposing_clef: ClefType
}

impl ClefTypeGroup {
	pub fn new(concert_clef: ClefType, transposing_clef: ClefType) -> Self { Self { concert_clef, transposing_clef}}
	pub fn from_clef(concert_clef: ClefType) -> Self { Self::new(concert_clef, concert_clef) }

	pub fn concert_clef(&self) -> ClefType { self.concert_clef }
	pub fn transposing_clef(&self) -> ClefType { self.transposing_clef }
	pub fn set_concert_clef(&mut self, v: ClefType) { self.concert_clef = v }
	pub fn set_transposing_clef(&mut self, v: ClefType) { self.transposing_clef = v }
}

impl Default for ClefTypeGroup {
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