use crate::*;
use crate::Color;

#[derive(Clone, Debug)]
pub struct Polyline {
	points: Vec<Point2F>,
	stroke: Stroke,
	color: Color,
}

impl Polyline {
	pub fn new(points: Vec<Point2F>, stroke: Stroke, color: Color) -> Self {
		Self { points, stroke, color}
	}

	pub fn points(&self) -> &Vec<Point2F> { &self.points }
	pub fn with_points(mut self, v: Vec<Point2F>) -> Self { self.points = v; self }

	pub fn stroke(&self) -> &Stroke { &self.stroke }
	pub fn with_stroke(mut self, v: Stroke) -> Self { self.stroke = v; self }

	pub fn color(&self) -> Color { self.color }
	pub fn with_color(mut self, v: Color) -> Self { self.color = v; self }
}

#[derive(Clone, Debug)]
pub struct Stroke {
	width: f32,
	start_cap: LineCap,
	end_cap: LineCap,
}

impl Default for Stroke {
	fn default() -> Self { Self {
		width: 1.0,
		start_cap: LineCap::None,
		end_cap: LineCap::None
	}}
}

impl Stroke {
	pub fn width(&self) -> f32 { self.width }
	pub fn with_width(mut self, v: f32) -> Self { self.width = v; self }

	pub fn start_cap(&self) -> &LineCap { &self.start_cap }
	pub fn with_startcap(mut self, v: LineCap) -> Self { self.start_cap = v; self }

	pub fn end_cap(&self) -> &LineCap { &self.end_cap }
	pub fn with_endcap(mut self, v: LineCap) -> Self { self.end_cap = v; self }
}

#[derive(Clone, Copy, Debug)]
pub enum LineCap {
	None,
	Round,
}