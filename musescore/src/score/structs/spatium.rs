use std::ops::{Mul, Div, Add, Sub, Neg, DivAssign, AddAssign, SubAssign};
use bitflags::_core::cmp::Ordering;
use bitflags::_core::ops::MulAssign;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Spatium(pub f32);

impl Eq for Spatium {

}

impl Ord for Spatium {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}

impl Default for Spatium {
	fn default() -> Self {
		Self(0.0)
	}
}

impl Spatium {
	pub fn points(self, base: f32) -> f32 {
		(self * base).0
	}
	pub fn ceil(self) -> Self { Spatium(self.0.ceil()) }
	pub fn floor(self) -> Self { Spatium(self.0.floor()) }
	pub fn abs(self) -> Self { Spatium(self.0.abs()) }
}

impl Into<f32> for Spatium {
	fn into(self) -> f32 { self.0 }
}

impl Add for Spatium {
	type Output = Spatium;

	fn add(self, rhs: Self) -> Self::Output { Spatium(self.0 + rhs.0) }
}

impl AddAssign for Spatium {
	fn add_assign(&mut self, rhs: Spatium) {
		self.0 += rhs.0
	}
}

impl Sub for Spatium {
	type Output = Spatium;

	fn sub(self, rhs: Self) -> Self::Output { Spatium(self.0 + rhs.0) }
}

impl SubAssign for Spatium {
	fn sub_assign(&mut self, rhs: Spatium) {
		self.0 -= rhs.0
	}
}

impl Neg for Spatium {
	type Output = Spatium;

	fn neg(self) -> Self::Output { Spatium(-self.0) }
}

impl<T: Into<f32>> Mul<T> for Spatium {
	type Output = Spatium;

	fn mul(self, rhs: T) -> Self::Output { Spatium(self.0 * rhs.into())}
}

impl<T: Into<f32>> MulAssign<T> for Spatium {
	fn mul_assign(&mut self, rhs: T) {
		self.0 *= rhs.into()
	}
}

impl<T: Into<f32>> Div<T> for Spatium {
	type Output = Spatium;

	fn div(self, rhs: T) -> Self::Output { Spatium(self.0 / rhs.into())}
}

impl<T: Into<f32>> DivAssign<T> for Spatium {
	fn div_assign(&mut self, rhs: T) {
		self.0 /= rhs.into()
	}
}