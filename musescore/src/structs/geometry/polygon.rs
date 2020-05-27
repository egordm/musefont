use crate::{Point2F, RectF, RectTrait};
use std::f32::{MIN, MAX};

pub struct PolygonF {
	points: Vec<Point2F>
}

impl PolygonF {
	pub fn new() -> Self { Self { points: Vec::new() }}

	pub fn push(&mut self, point: Point2F) {
		self.points.push(point)
	}

	pub fn bbox(&self) -> RectF {
		let (mut minx, mut miny, mut maxx, mut maxy) = (MAX, MAX, MIN, MIN);
		for p in self.points.iter() {
			minx = minx.min(p.x);
			maxx = maxx.max(p.x);
			miny = miny.min(p.y);
			maxy = maxy.max(p.y);
		}
		RectF::from_ps(Point2F::new(minx, miny), Point2F::new(miny, maxy))
	}
}