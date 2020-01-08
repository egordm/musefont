use crate::font::*;

#[derive(Clone, Debug)]
pub struct KeySigEvent {
	key: Key,
	mode: KeyMode,
	custom: bool,
	key_symbols: Vec<KeySym>,
}

impl Default for KeySigEvent {
	fn default() -> Self { Self {
		key: Key::Invalid,
		mode: KeyMode::Unknown,
		custom: false,
		key_symbols: vec![]
	}}
}

impl KeySigEvent {
	pub fn from_key(key: Key) -> Self {
		let mut res = Self::default();
		res.set_key(key);
		res
	}

	pub fn key(&self) -> Key { self.key }
	pub fn mode(&self) -> KeyMode { self.mode }
	pub fn set_mode(&mut self, v: KeyMode) { self.mode = v; }
	pub fn custom(&self) -> bool { self.custom}
	pub fn is_valid(&self) -> bool { self.key != Key::Invalid }
	pub fn is_atonal(&self) -> bool { self.mode == KeyMode::None }
	pub fn key_symbols(&self) -> &Vec<KeySym> { &self.key_symbols }

	pub fn set_key(&mut self, key: Key) {
		self.key = key;
		self.custom = false;
	}
}

#[derive(Clone, Copy, Debug)]
pub struct KeySym {
	sym: SymId,
	spos: Point2F, // position in spatium units
	pos: Point2F,  // actual pixel position on screen (set by layout)
}

pub const KEY_C_B: isize = -7;
pub const KEY_G_B: isize = -6;
pub const KEY_D_B: isize = -5;
pub const KEY_A_B: isize = -4;
pub const KEY_E_B: isize = -3;
pub const KEY_B_B: isize = -2;
pub const KEY_F: isize = -1;
pub const KEY_C: isize = 0;
pub const KEY_G: isize = 1;
pub const KEY_D: isize = 2;
pub const KEY_A: isize = 3;
pub const KEY_E: isize = 4;
pub const KEY_B: isize = 5;
pub const KEY_F_S: isize = 6;
pub const KEY_C_S: isize = 7;
pub const KEY_DELTA_ENHARMONIC: isize = 12;
pub const KEY_INVALID: isize = 13;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Key {
	C_B = KEY_C_B,
	G_B = KEY_G_B,
	D_B = KEY_D_B,
	A_B = KEY_A_B,
	E_B = KEY_E_B,
	B_B = KEY_B_B,
	F = KEY_F,
	C = KEY_C,
	G = KEY_G,
	D = KEY_D,
	A = KEY_A,
	E = KEY_E,
	B = KEY_B,
	F_S = KEY_F_S,
	C_S = KEY_C_S,
	DeltaEnharmonic = KEY_DELTA_ENHARMONIC,
	Invalid = KEY_INVALID
}

#[derive(Clone, Copy, Debug, PartialEq, Primitive, Eq, Hash)]
pub enum KeyMode {
	Unknown = 0,
	None = 1,
	Major = 2,
	Minor = 3,
	Dorian = 4,
	Phrygian = 5,
	Lydian = 6,
	Mixolydian = 7,
	Aeolian = 8,
	Ionian = 9,
	Locrian = 10,
}

pub const KEY_MIN: Key = Key::C_B;
pub const KEY_MAX: Key = Key::C_S;
pub const KEY_COUNT: u32 = 15;
