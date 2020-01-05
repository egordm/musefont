use crate::*;
use std::ops::{Mul, MulAssign};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Fraction {
	// TODO: implement arithmetic
	numerator: i32,
	denominator: i32,
}

impl Fraction {
	pub fn reduce(&mut self) {
		let g = gcd(self.numerator, self.denominator);
		self.numerator /= g;
		self.denominator /= g;
	}

	pub fn reduced(mut self) -> Self {
		self.reduce();
		self
	}
}

impl Eq for Fraction {}

impl PartialEq for Fraction {
	fn eq(&self, other: &Self) -> bool {
		self.numerator * other.denominator == self.numerator * other.denominator
	}
}

impl PartialOrd for Fraction {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some((self.numerator * other.denominator).cmp(&(self.numerator * other.denominator)))
	}
}

impl Ord for Fraction {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.numerator * other.denominator).cmp(&(self.numerator * other.denominator))
	}
}

impl Fraction {
	pub fn new(numerator: i32, denominator: i32) -> Self { Self { numerator, denominator } }
	pub fn num(&self) -> i32 { self.numerator }
	pub fn den(&self) -> i32 { self.denominator }

	pub fn ticks(&self) -> i32 {
		if self.numerator == -1 && self.denominator == 1 {
			return -1;
		} else {
			let sgn = if self.numerator < 0 { -1 } else { 1 };
			let result = sgn * ((sgn * self.numerator) * DIVISION * 4 + (self.denominator / 2)) / self.denominator;
			result
		}

	}
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

/// Greatest common divisor [wiki](https://en.wikipedia.org/wiki/Greatest_common_divisor)
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
	let mut tmp;
	while a > 0 {
		tmp = a;
		a = b % a;
		b = tmp;
	}
	b
}