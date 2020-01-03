use crate::num_traits::FromPrimitive;
use crate::*;
use crate::score::*;
use num_traits::abs;

/// # [Hook](https://en.wikipedia.org/wiki/Note_value)
#[derive(Debug, Clone)]
pub struct Hook {
	element: Symbol,
	hook_type: HookType,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum HookType {
	None = 0,
	Flag8thUp = 1,
	Flag16thUp = 2,
	Flag32ndUp = 3,
	Flag64thUp = 4,
	Flag128thUp = 5,
	Flag256thUp = 6,
	Flag512thUp = 7,
	Flag1024thUp = 8,
	Flag8thDown = 9,
	Flag16thDown = 10,
	Flag32ndDown = 11,
	Flag64thDown = 12,
	Flag128thDown = 13,
	Flag256thDown = 14,
	Flag512thDown = 15,
	Flag1024thDown = 16,
}

impl HookType {
	pub fn down(self) -> HookType {
		if self == HookType::None { self } else if self as u32 <= 8 { HookType::from_u32(self as u32 + 8).unwrap() } else { self }
	}

	pub fn up(self) -> HookType {
		if self == HookType::None { self } else if self as u32 > 8 { HookType::from_u32(self as u32 - 8).unwrap() } else { self }
	}

	pub fn count(self) -> i32 {
		abs(self.index())
	}

	pub fn index(self) -> i32 {
		let ret = self as i32;
		if ret > 8 { -(ret - 8) } else { ret }
	}

	pub fn from_index(mut idx: i32) -> Self {
		if idx < 0 { idx = -idx + 8 }
		HookType::from_i32(idx).unwrap()
	}
}


