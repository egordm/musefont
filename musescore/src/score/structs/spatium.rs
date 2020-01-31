use std::ops::{Mul, Div, Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Spatium(pub f32);

impl Spatium {
	pub fn points(self, base: f32) -> f32 {
		(self * base).0
	}
}

impl Into<f32> for Spatium {
	fn into(self) -> f32 { self.0 }
}

impl Add for Spatium {
	type Output = Spatium;

	fn add(self, rhs: Self) -> Self::Output { Spatium(self.0 + rhs.0) }
}

impl Sub for Spatium {
	type Output = Spatium;

	fn sub(self, rhs: Self) -> Self::Output { Spatium(self.0 + rhs.0) }
}

impl<T: Into<f32>> Mul<T> for Spatium {
	type Output = Spatium;

	fn mul(self, rhs: T) -> Self::Output { Spatium(self.0 * rhs.into())}
}

impl<T: Into<f32>> Div<T> for Spatium {
	type Output = Spatium;

	fn div(self, rhs: T) -> Self::Output { Spatium(self.0 / rhs.into())}
}