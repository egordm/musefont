use crate::*;

use std::convert::TryInto;
use num_traits::{FromPrimitive, ToPrimitive};
use crate::score::Spatium;


macro_rules! decl_vvariant {{
	enum ValueVariant
	{
		$($Variant:ident($Type:ty) -> $fn_ty:ident),* $(,)*
	}
} => {
	#[derive(Clone, Debug, PartialEq)]
	pub enum ValueVariant {
		None,
		$($Variant($Type)),*
	}

	impl ValueVariant {
		$(pub fn $fn_ty(self) -> $Type { self.expect::<$Type>() })*
	}

	$(
	impl<'a> TryInto<&'a $Type> for &'a ValueVariant {
		type Error = ();
		fn try_into(self) -> Result<&'a $Type, Self::Error> {
			if let ValueVariant::$Variant(v) = self { Ok(v) } else { Err(()) }
		}
	}
	impl TryInto<$Type> for ValueVariant {
		type Error = ();
		fn try_into(self) -> Result<$Type, Self::Error> {
			if let ValueVariant::$Variant(v) = self { Ok(v) } else { Err(()) }
		}
	}
	impl From<$Type> for ValueVariant {
		fn from(v: $Type) -> Self { ValueVariant::$Variant(v) }
	}
	impl From<&$Type> for ValueVariant where $Type: Clone {
		fn from(v: &$Type) -> Self { ValueVariant::$Variant(v.clone()) }
	}
	)*
}}

/*pub trait ExpectInto<T>: Sized {
	fn expecti(self) -> Result<T, Self::Error>;
}*/

decl_vvariant!{
	enum ValueVariant {
		Spatium(Spatium) -> spt,
		Bool(bool)       -> bool,
		Float(f32)       -> flt,
		Int(i32)         -> int,
		UInt(u32)        -> uint,
		Point(Point2F)   -> pt,
		String(String)   -> str,
		Color(Color)     -> color
	}
}

impl ValueVariant {
	pub fn value<T>(self) -> Option<T> where Self: TryInto<T> {
		self.try_into().ok()
	}
	pub fn expect<T>(self) -> T where Self: TryInto<T> {
		self.try_into().ok().expect("Expected ValueVariant to be _ found _")
	}
	pub fn expect_ref<'a, T>(&'a self) -> &'a T where &'a Self: TryInto<&'a T> {
		self.try_into().ok().expect("Expected ValueVariant to be _ found _")
	}

	pub fn with_value<T, F: FnMut(T)>(self, mut f: F) -> bool where ValueVariant: TryInto<T> {
		if let Ok(v) = self.try_into() { f(v); true } else { false }
	}

	pub fn from_enum<T: ToPrimitive>(v: T) -> Self {
		if let Some(v) = v.to_u32() { ValueVariant::UInt(v) }
		else if let Some(v) = v.to_i32() { ValueVariant::Int(v) }
		else { ValueVariant::None }
	}
	pub fn with_enum<T: FromPrimitive, F: FnMut(T)>(self, mut f: F) -> bool {
		if let Ok(v) = self.try_into_enum() { f(v); true } else { false }
	}
	pub fn try_into_enum<T: FromPrimitive>(self) -> Result<T, ()> {
		if let ValueVariant::UInt(v) = self {
			if let Some(v) = T::from_u32(v) { return Ok(v); }
		}
		if let ValueVariant::Int(v) = self {
			if let Some(v) = T::from_i32(v) { return Ok(v); }
		}
		Err(())
	}

	pub fn if_none<F: FnMut() -> ValueVariant>(self, mut f: F) -> ValueVariant {
		if self == ValueVariant::None { f() } else { self }
	}
}

impl Default for ValueVariant {
	fn default() -> Self { ValueVariant::None }
}


impl TryInto<u16> for ValueVariant {
	type Error = ();
	fn try_into(self) -> Result<u16, Self::Error> {
		if let ValueVariant::UInt(v) = self { Ok(v as u16) } else { Err(()) }
	}
}
impl From<u16> for ValueVariant {
	fn from(v: u16) -> Self { ValueVariant::UInt(v as u32) }
}