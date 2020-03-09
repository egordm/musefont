use crate::num_traits::FromPrimitive;
use crate::*;
use crate::score::*;
use num_traits::abs;
use crate::font::SymName;

/// # [Hook](https://en.wikipedia.org/wiki/Note_value)
#[derive(Debug, Clone)]
pub struct Hook {
	element: Symbol,
	hook_type: HookType,
}

impl Hook {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: Symbol::default(score),
		hook_type: HookType::None,
	})}

	pub fn hook_type(&self) -> HookType { self.hook_type }
	pub fn set_hook_type(&mut self, v: HookType) {
		self.hook_type = v;
		self.element.set_sym(match self.hook_type {
			HookType::None => SymName::NoSym,
			HookType::Flag8thUp => SymName::Flag8thUp,
			HookType::Flag16thUp => SymName::Flag16thUp,
			HookType::Flag32ndUp => SymName::Flag32ndUp,
			HookType::Flag64thUp => SymName::Flag64thUp,
			HookType::Flag128thUp => SymName::Flag128thUp,
			HookType::Flag256thUp => SymName::Flag256thUp,
			HookType::Flag512thUp => SymName::Flag512thUp,
			HookType::Flag1024thUp => SymName::Flag1024thUp,
			HookType::Flag8thDown => SymName::Flag8thDown,
			HookType::Flag16thDown => SymName::Flag16thDown,
			HookType::Flag32ndDown => SymName::Flag32ndDown,
			HookType::Flag64thDown => SymName::Flag64thDown,
			HookType::Flag128thDown => SymName::Flag128thDown,
			HookType::Flag256thDown => SymName::Flag256thDown,
			HookType::Flag512thDown => SymName::Flag512thDown,
			HookType::Flag1024thDown => SymName::Flag1024thDown,
		})
	}

	pub fn sym(&self) -> SymName {
		self.element.sym()
	}
}

impl Element for Hook {
	fn el_data(&self) -> &ElementData { self.element.el_data() }
	fn el_data_mut(&mut self) -> &mut ElementData { self.element.el_data_mut() }

	fn element_type(&self) -> ElementType { ElementType::Hook }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.element.get_custom_property(p)
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.element.set_custom_property(p, v.clone()) || self.set_element_property(p, v)
	}
}

impl AtomTrait for Hook {

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


