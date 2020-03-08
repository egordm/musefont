pub type Point2F = euclid::default::Point2D<f32>;
pub type Vec2F = euclid::default::Vector2D<f32>;
pub type Size2F = euclid::default::Size2D<f32>;
pub type Size2U = euclid::default::Size2D<u32>;
pub type RectF = euclid::default::Rect<f32>;

pub type Color = [u8; 4];
pub const COLOR_BLACK: Color = [0x00, 0x00, 0x00, 0xFF];
pub const COLOR_RED: Color = [0xFF, 0x00, 0x00, 0xFF];
pub const COLOR_GREEN: Color = [0x00, 0xFF, 0x00, 0xFF];
pub const COLOR_BLUE: Color = [0x00, 0x00, 0xFF, 0xFF];

pub const VEC_ZERO: Vec2F = Vec2F::new(0., 0.);
pub const POINT_ZERO: Point2F = Point2F::new(0., 0.);
pub const SIZE_ONE: Size2F = Size2F::new(1., 1.);

pub trait RectTrait {
	fn right(&self) -> f32;
	fn left(&self) -> f32;
	fn up(&self) -> f32;
	fn top(&self) -> f32 { self.up() }
	fn down(&self) -> f32;
	fn bottom(&self) -> f32 { self.down() }

	fn from_ps(p1: Point2F, p2: Point2F) -> Self;

	fn norm(self) -> Self;
	fn adjust(self, p1: Point2F, p2: Point2F) -> Self;
}

impl RectTrait for RectF {
	fn right(&self) -> f32 { self.origin.x + self.size.width}
	fn left(&self) -> f32 { self.origin.x }
	fn up(&self) -> f32 { self.origin.y + self.size.height }
	fn down(&self) -> f32 {  self.origin.y }

	fn from_ps(p1: Point2F, p2: Point2F) -> Self {
		let size = Size2F::new( (p1.x - p2.x).abs(), (p1.y - p2.y).abs());
		let origin = Point2F::new(p1.x.min(p2.x), p1.y.min(p2.y));
		RectF::new(origin, size)
	}
	fn norm(mut self) -> Self {
		if self.size.width < 0. {
			self.origin.x += self.size.width;
			self.size.width = -self.size.width;
		}
		if self.size.height < 0. {
			self.origin.y += self.size.height;
			self.size.height = -self.size.height;
		}
		self
	}
	fn adjust(self, p1: Point2F, p2: Point2F) -> Self {
		let wh = p2 - p1;
		RectF::new(
			self.origin + p1.to_vector(),
			Size2F::new(self.size.width + wh.x, self.size.height + wh.y)
		)
	}
}

pub fn size_from(v: f32) -> Size2F {
	Size2F::new(v, v)
}

pub fn scale_pos(p: Point2F, s: Size2F) -> Point2F {
	Point2F::new(p.x * s.width, p.y * s.height)
}
