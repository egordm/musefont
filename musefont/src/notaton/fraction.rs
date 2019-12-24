use std::ops::{Mul, MulAssign};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fraction {
	// TODO: implement arithmetic
	numerator: i32,
	denominator: i32,
}

impl Fraction {
	pub fn new(numerator: i32, denominator: i32) -> Self { Self { numerator, denominator } }
	pub fn num(&self) -> i32 { self.numerator }
	pub fn den(&self) -> i32 { self.denominator }
}

impl Mul for Fraction {
	type Output = Fraction;

	fn mul(self, rhs: Self) -> Self::Output {
		Fraction::new(self.num() * rhs.num(), self.den() * rhs.den())
	}
}

impl MulAssign for Fraction {
	fn mul_assign(&mut self, rhs: Self) {
		self.numerator *= rhs.num();
		self.denominator *= rhs.den();
	}
}