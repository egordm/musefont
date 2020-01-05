use std::ops::{Add, Mul};

pub type Point2F = euclid::default::Point2D<f32>;
pub type Vec2F = euclid::default::Vector2D<f32>;
pub type Size2F = euclid::default::Size2D<f32>;
pub type Size2U = euclid::default::Size2D<u32>;
pub type RectF = euclid::default::Rect<f32>;

pub fn rect_from_points(p1: Point2F, p2: Point2F) -> RectF {
	let size = Size2F::new( (p1.x - p2.x).abs(), (p1.y - p2.y).abs());
	let origin = Point2F::new(p1.x.min(p2.x), p1.y.min(p2.y));
	RectF::new(origin, size)
}

pub fn rect_norm(mut r: RectF) -> RectF {
	if r.size.width < 0. {
		r.origin.x += r.size.width;
		r.size.width = -r.size.width;
	}
	if r.size.height < 0. {
		r.origin.y += r.size.height;
		r.size.height = -r.size.height;
	}
	r
}

pub fn rect_adjust(r: RectF, p1: Point2F, p2: Point2F) -> RectF {
	let wh = p2 - p1;
	RectF::new(r.origin + p1.to_vector(), Size2F::new(r.size.width + wh.x, r.size.height + wh.y))
}

pub fn scale_pos(p: Point2F, s: Size2F) -> Point2F {
	Point2F::new(p.x * s.width, p.y * s.height)
}

#[derive(Clone, Debug)]
pub struct LineF {
	pub p1: Point2F,
	pub p2: Point2F,
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

pub const POINT_ZERO: Point2F = Point2F::new(0., 0.);
pub const SIZE_ONE: Size2F = Size2F::new(1., 1.);

pub type RasterizationOptions = font_kit::canvas::RasterizationOptions;
pub type Format = font_kit::canvas::Format;

macro_rules! impl_display {
    ($enum:ident, {$($variant:pat => $fmt_string:expr),+$(,)* }) => {
        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use self::$enum::*;
                match &self {
                    $(
                        $variant => write!(f, "{}", $fmt_string),
                    )+
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum FontLoadingError {
	IO(std::io::Error),
	Font(font_kit::error::FontLoadingError),
	Glyph(font_kit::error::GlyphLoadingError),
	Json(json::Error),
	Other(String),
}

impl std::error::Error for FontLoadingError {}

impl_display! { FontLoadingError, {
        IO(e) => format!("IO error: {}", e),
        Font(e) => format!("Font loading error: {}", e),
        Glyph(e) => format!("Glyph loading error: {}", e),
        Json(e) => format!("Json loading error: {}", e),
        Other(e) => format!("Error occurred: {}", e),
    }
}