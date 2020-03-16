use crate::score::Spatium;
use std::ops::{Add, SubAssign, Sub};
use crate::score::PropertyId::LineStyle;
use bitflags::_core::ops::AddAssign;

/// Represents a staff line as a means of counting
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Line(i32);

impl Line {
	pub fn value(&self) -> Spatium { Spatium(self.0 as f32 / 2.) }

	pub fn is_half_step(&self) -> bool { (self.0 & 1) != 0 }
	pub fn ceil(self) -> Line {
		if self.is_half_step() { Line(self.0 + 1) }
		else { Line(self.0) }
	}
	pub fn floor(self) -> Line {
		if self.is_half_step() { Line(self.0 - 1) }
		else { Line(self.0) }
	}
}

impl From<Spatium> for Line {
	fn from(v: Spatium) -> Self { v.0.into() }
}

impl From<i32> for Line {
	fn from(v: i32) -> Self { Line((v as f32 * 2.) as i32) }
}

impl From<f32> for Line {
	fn from(v: f32) -> Self { Line((v * 2.) as i32) }
}

impl Default for Line {
	fn default() -> Self { Self(0) }
}

impl AddAssign<Line> for Line {
	fn add_assign(&mut self, rhs: Line) {
		self.0 += rhs.0
	}
}

impl Add<Line> for Line {
	type Output = Line;

	fn add(self, rhs: Line) -> Self::Output {
		Line(self.0 + rhs.0)
	}
}

impl SubAssign<Line> for Line {
	fn sub_assign(&mut self, rhs: Line) {
		self.0 -= rhs.0
	}
}

impl Sub<Line> for Line {
	type Output = Line;

	fn sub(self, rhs: Line) -> Self::Output {
		Line(self.0 - rhs.0)
	}
}