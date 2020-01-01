pub type Point2F = euclid::default::Point2D<f32>;
pub type Vec2F = euclid::default::Vector2D<f32>;
pub type Size2F = euclid::default::Size2D<f32>;
pub type Size2U = euclid::default::Size2D<u32>;
pub type RectF = euclid::default::Rect<f32>;

pub const VEC_ZERO: Vec2F = Vec2F::new(0., 0.);
pub const POINT_ZERO: Point2F = Point2F::new(0., 0.);
pub const SIZE_ONE: Size2F = Size2F::new(1., 1.);

pub fn rect_from_ps(p1: Point2F, p2: Point2F) -> RectF {
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
	RectF::new(
		r.origin + p1.to_vector(),
		Size2F::new(r.size.width + wh.x, r.size.height + wh.y)
	)
}

pub fn scale_pos(p: Point2F, s: Size2F) -> Point2F {
	Point2F::new(p.x * s.width, p.y * s.height)
}
