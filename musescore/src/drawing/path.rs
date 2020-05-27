use crate::Vec2F;
use crate::drawing::Instruction;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineCap {
	Butt,
	Square,
	Round,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineJoin {
	Miter(f32),
	Bevel,
	Round,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DashStyle {
	dash_sizes: Vec<f32>,
	dash_offsets: Vec<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
	start: Vec2F,
	segments: Vec<Segment>,
	line_width: f32,
	line_cap: LineCap,
	line_join: LineJoin,
	dash_style: Option<DashStyle>,
	fill: bool,
}

impl Path {
	pub fn new() -> Self { Self {
		start: Vec2F::default(),
		segments: vec![],
		line_width: 1.0,
		line_cap: LineCap::Butt,
		line_join: LineJoin::Miter(10.),
		dash_style: None,
		fill: false,
	}}

	pub fn from_line(p1: Vec2F, p2: Vec2F) -> Self {
		Path::new().move_to(p1).add_segment(Segment::Line(p2))
	}

	pub fn translate(mut self, m: Vec2F) -> Self {
		self.start += m;
		for s in self.segments.iter_mut() { s.translate(m) }
		self
	}

	pub fn segments(&self) -> &Vec<Segment> { &self.segments }
	pub fn add_segment(mut self, s: Segment) -> Self { self.segments.push(s); self }

	pub fn start_pos(&self) -> &Vec2F { &self.start }
	pub fn move_to(mut self, start: Vec2F) -> Self { self.start = start; self }

	pub fn line_width(&self) -> f32 { self.line_width }
	pub fn set_line_width(mut self, line_width: f32) -> Self { self.line_width = line_width; self }

	pub fn line_cap(&self) -> LineCap { self.line_cap }
	pub fn set_line_cap(mut self, line_cap: LineCap) -> Self { self.line_cap = line_cap; self }

	pub fn line_join(&self) -> LineJoin { self.line_join }
	pub fn set_line_join(mut self, line_join: LineJoin) -> Self { self.line_join = line_join; self }

	pub fn fill(&self) -> bool { self.fill }
	pub fn set_fill(mut self, fill: bool) -> Self { self.fill = fill; self }
}

impl Into<Instruction> for Path {
	fn into(self) -> Instruction { Instruction::Path(self) }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Segment {
	Line(Vec2F),
	QuadraticCurve { ctrl: Vec2F, to: Vec2F },
	BezierCurve { ctrl1: Vec2F, ctrl2: Vec2F, to: Vec2F }
}

impl Segment {
	pub fn translate(&mut self, m: Vec2F) {
		match self {
			Segment::Line(p) => { *p += m; },
			Segment::QuadraticCurve { ctrl, to } => { *ctrl += m; *to += m; },
			Segment::BezierCurve { ctrl1, ctrl2, to } => { *ctrl1 += m; *ctrl2 += m; *to += m; },
		}
	}
}