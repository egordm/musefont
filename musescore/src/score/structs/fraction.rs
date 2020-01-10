use crate::*;
use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};
use std::cmp::Ordering;

pub type TimeTicks = i32;
pub const INVALID_TIME: TimeTicks = -1;

#[derive(Clone, Copy, Debug)]
pub struct Fraction {
	// TODO: implement arithmetic
	pub numerator: i32,
	pub denominator: i32,
}

impl Default for Fraction {
	fn default() -> Self { Self::new(0, 1)}
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

impl Into<i32> for Fraction {
	fn into(self) -> i32 { self.ticks() }
}

impl From<i32> for Fraction {
	fn from(v: i32) -> Self { Self::from_ticks(v) }
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
			let result = sgn * ((sgn * self.numerator) * constants::DIVISION * 4 + (self.denominator / 2)) / self.denominator;
			result
		}
	}

	pub fn from_ticks(v: i32) -> Self {
		if v == -1 { Fraction::new(-1, 1) }
		else { Fraction::new(v, constants::DIVISION * 4).reduced() }
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

impl Div for Fraction {
	type Output = Fraction;

	fn div(mut self, rhs: Self) -> Self::Output {
		self /= rhs;
		return self;
	}
}

impl DivAssign for Fraction {
	fn div_assign(&mut self, rhs: Self) {
		let sign = if rhs.numerator >= 0 { 1 } else { -1 };
		self.numerator *= sign * rhs.denominator;
		self.denominator *= sign * rhs.numerator;
		if self.numerator != sign { self.reduce() }
	}
}

impl Add for Fraction {
	type Output = Fraction;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}


impl AddAssign for Fraction {
	fn add_assign(&mut self, rhs: Self) {
		if self.denominator == rhs.denominator {
			self.numerator += rhs.numerator; // Common enough use case to be handled separately for efficiency
		} else {
			let g = gcd(self.denominator, rhs.denominator);
			let m1 = rhs.denominator / g; // This saves one division over straight lcm
			self.numerator = self.numerator * m1 + rhs.numerator * (self.denominator / g);
			self.denominator *= m1;
		}
	}
}

impl Sub for Fraction {
	type Output = Fraction;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self -= rhs;
		self
	}
}

impl SubAssign for Fraction {
	fn sub_assign(&mut self, rhs: Self) {
		if self.denominator == rhs.denominator {
			self.numerator -= rhs.numerator; // Common enough use case to be handled separately for efficiency
		} else {
			let g = gcd(self.denominator, rhs.denominator);
			let m1 = rhs.denominator / g; // This saves one division over straight lcm
			self.numerator = self.numerator * m1 - rhs.numerator * (self.denominator / g);
			self.denominator *= m1;
		}
	}
}

/// Greatest common divisor [wiki](https://en.wikipedia.org/wiki/Greatest_common_divisor)
fn gcd(mut a: i32, mut b: i32) -> i32 {
	let mut tmp;
	while a > 0 {
		tmp = a;
		a = b % a;
		b = tmp;
	}
	b
}