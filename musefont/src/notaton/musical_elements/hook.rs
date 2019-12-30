use crate::num_traits::FromPrimitive;
use crate::*;

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
		if self == HookType::None { self }
		else if self as u32 <= 8 { HookType::from_u32(self as u32 + 8).unwrap() }
		else { self }
	}

	pub fn up(self) -> HookType {
		if self == HookType::None { self }
		else if self as u32 > 8 { HookType::from_u32(self as u32 - 8).unwrap() }
		else { self }
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

#[derive(Clone, Debug)]
pub struct Hook {
	element: Symbol,
	hook_type: HookType,
}

impl Hook {
	pub fn new(score: Score) -> Elem<Self> { Elem::new(Self {
		element: Symbol::default(score),
		hook_type: HookType::None
	})}

	pub fn hook_type(&self) -> HookType { self.hook_type }
	pub fn set_hook_type(&mut self, v: HookType) {
		self.hook_type = v;
		self.element.set_sym(match self.hook_type {
			HookType::None => SymIdent::NoSym,
			HookType::Flag8thUp => SymIdent::Flag8thUp,
			HookType::Flag16thUp => SymIdent::Flag16thUp,
			HookType::Flag32ndUp => SymIdent::Flag32ndUp,
			HookType::Flag64thUp => SymIdent::Flag64thUp,
			HookType::Flag128thUp => SymIdent::Flag128thUp,
			HookType::Flag256thUp => SymIdent::Flag256thUp,
			HookType::Flag512thUp => SymIdent::Flag512thUp,
			HookType::Flag1024thUp => SymIdent::Flag1024thUp,
			HookType::Flag8thDown => SymIdent::Flag8thDown,
			HookType::Flag16thDown => SymIdent::Flag16thDown,
			HookType::Flag32ndDown => SymIdent::Flag32ndDown,
			HookType::Flag64thDown => SymIdent::Flag64thDown,
			HookType::Flag128thDown => SymIdent::Flag128thDown,
			HookType::Flag256thDown => SymIdent::Flag256thDown,
			HookType::Flag512thDown => SymIdent::Flag512thDown,
			HookType::Flag1024thDown => SymIdent::Flag1024thDown,
		} as SymId)
	}
}

impl ElementTrait for Hook {
	fn el(&self) -> &Element {&self.element.el() }
	fn el_mut(&mut self) -> &mut Element { self.element.el_mut() }
	fn element_type(&self) -> ElementType { ElementType::Hook }

	fn scale(&self) -> f32 { self.parent().as_ref().map(ElementTrait::scale).unwrap_or_default()}
}

impl Drawable for Hook {
	fn layout(&mut self) {
		self.element.layout()
	}

	fn draw(&self, painter: PainterRef) {
		self.element.draw(painter)
	}
}