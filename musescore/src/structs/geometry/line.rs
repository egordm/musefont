use std::ops::{Add, Mul};
use super::*;

#[derive(Clone, Debug)]
pub struct LineF {
	pub p1: Point2F,
	pub p2: Point2F,
}

impl Default for LineF {
	fn default() -> Self { LineF::new(Point2F::default(), Point2F::default()) }
}

impl LineF {
	pub fn new(p1: Point2F, p2: Point2F) -> Self { Self {p1, p2}}

	pub fn len(&self) -> f32 {
		let diff = self.p1 - self.p2;
		diff.dot(diff).sqrt()
	}

	pub fn x1(&self) -> f32 { self.p1.x }
	pub fn y1(&self) -> f32 { self.p1.y }
	pub fn x2(&self) -> f32 { self.p2.x }
	pub fn y2(&self) -> f32 { self.p2.y }
}

impl Add<Vec2F> for LineF {
	type Output = LineF;

	fn add(mut self, rhs: Vec2F) -> Self::Output {
		self.p1 += rhs;
		self.p2 += rhs;
		self
	}
}

impl Mul<f32> for LineF {
	type Output = LineF;

	fn mul(mut self, rhs: f32) -> Self::Output {
		self.p1 *= rhs;
		self.p2 *= rhs;
		self
	}
}

impl Mul<Size2F> for LineF {
	type Output = LineF;

	fn mul(mut self, rhs: Size2F) -> Self::Output {
		self.p1 = scale_pos(self.p1, rhs);
		self.p2 = scale_pos(self.p2, rhs);
		self
	}
}