#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
	C_B = -7,
	G_B = -6,
	D_B = -5,
	A_B = -4,
	E_B = -3,
	B_B = -2,
	F = -1,
	C = 0,
	G = 1,
	D = 2,
	A = 3,
	E = 4,
	B = 5,
	F_S = 6,
	C_S = 7,
	DeltaEnharmonic = 12
}

pub const KEY_MIN: Key = Key::C_B;
pub const KEY_MAX: Key = Key::C_S;
pub const KEY_COUNT: u32 = 15;
