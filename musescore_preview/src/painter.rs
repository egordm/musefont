use crate::window::Canvas;
use musescore::{Vec2F, Color, drawing::*};
use pathfinder_color::ColorU;
use pathfinder_content::stroke;
use pathfinder_canvas as canvas;
use pathfinder_geometry::vector::Vector2F;
use pathfinder_geometry::transform2d::Transform2F;
use musescore::font::ScoreFont;
use std::f32::consts::PI;

pub struct PfPainter<'a> {
	canvas: &'a mut Canvas,
	pos: Vec2F,
	color: Color,
	symbol_font: Option<ScoreFont>,
	world_scale: f32,
	local_scale: f32,
}

impl<'a> PfPainter<'a> {
	pub fn new(canvas: &'a mut Canvas) -> Self { Self {
		canvas,
		pos: Default::default(),
		color: [0, 0, 0, 0xFF],
		symbol_font: None,
		world_scale: 1.,
		local_scale: 1.,
	}}

	fn convert_vec(&self, p: &Vec2F) -> Vector2F {
		let p = p.clone() + self.pos;
		Vector2F::new(self.scale() * p.x, self.scale() *  p.y)
	}

	fn world_scale(&self) -> f32 { self.world_scale }

	fn scale(&self) -> f32 { self.world_scale * self.local_scale }
}

impl<'a> Painter for PfPainter<'a> {
	fn draw(&mut self, i: Instruction) {
		match i {
			Instruction::Path(path) => {
				self.canvas.set_stroke_style(canvas::FillStyle::Color(convert_color(self.color.clone())));

				let (line_width, line_cap, line_join, miter_limit) = convert_path_style(&path);
				self.canvas.set_line_width(line_width * self.scale());
				self.canvas.set_line_cap(line_cap);
				self.canvas.set_line_join(line_join);
				self.canvas.set_miter_limit(miter_limit);

				let mut cpath = canvas::Path2D::new();
				cpath.move_to(self.convert_vec(path.start_pos()));
				for segment in path.segments() {
					match segment {
						Segment::Line(to) => cpath.line_to(self.convert_vec(to)),
						Segment::QuadraticCurve { ctrl, to } =>
							cpath.quadratic_curve_to(self.convert_vec(ctrl), self.convert_vec(to)),
						Segment::BezierCurve { ctrl1, ctrl2, to } =>
							cpath.bezier_curve_to(self.convert_vec(ctrl1), self.convert_vec(ctrl2), self.convert_vec(to)),
					}
				}
				self.canvas.stroke_path(cpath);
			},
			Instruction::Symbol(symbol) => {
				self.canvas.set_fill_style(canvas::FillStyle::Color(convert_color(self.color.clone())));
				// Pathfinder already dpi corrects it
				self.canvas.set_font_size(symbol.scale().width * self.local_scale); // TODO: this should not vary?

				let scale = Vector2F::new(1., symbol.scale().height / symbol.scale().width);
				let scale_inv = Vector2F::new(1. / scale.x(), 1. / scale.y());
				let pos = self.convert_vec(&symbol.pos().to_vector()).scale_xy(scale_inv);
				let cur_transform = self.canvas.current_transform();
				let new_transform = cur_transform * Transform2F::from_scale(scale);

				self.canvas.set_current_transform(&new_transform);
				self.canvas.fill_text(&symbol.sym_char().to_string(), pos);
				self.canvas.set_current_transform(&cur_transform);

			}
			Instruction::Rect(rect, stroke_width) => {
				self.canvas.set_stroke_style(canvas::FillStyle::Color(convert_color(self.color.clone())));
				let rect = pathfinder_geometry::rect::RectF::new(
					self.convert_vec(&rect.origin.to_vector()),
					Vector2F::new(rect.size.width * self.scale(), rect.size.height * self.scale()),
				);
				self.canvas.set_line_width(stroke_width);
				self.canvas.stroke_rect(rect);
			},
			Instruction::Point(pos, size) => {
				self.canvas.set_fill_style(canvas::FillStyle::Color(convert_color(self.color.clone())));
				let mut point = canvas::Path2D::new();
				let pos = self.convert_vec(&pos.to_vector());
				point.ellipse(pos, Vector2F::new(size, size), 0., 0., PI * 2.);
				self.canvas.fill_path(point)
			}
		}
	}

	fn translate(&mut self, m: Vec2F) { self.pos += m; }

	fn set_scale(&mut self, scale: f32) {
		self.local_scale = scale;
	}

	fn set_dpi(&mut self, dpi: f32) {
		self.world_scale = dpi / musescore::constants::DPI;
	}

	// TODO: probably should pass to the shader
	fn set_color(&mut self, c: Color) { self.color = c; }

	fn set_score_font(&mut self, f: ScoreFont) {
		self.canvas.set_font(f.font().clone());
		self.symbol_font = Some(f);
	}
}

fn convert_color(c: Color) -> ColorU { ColorU { r: c[0], g: c[1], b: c[2], a: c[3] } }

fn convert_path_style(path: &Path) -> (f32, stroke::LineCap, canvas::LineJoin, f32) {
	let line_width = path.line_width();
	let line_cap = match path.line_cap() {
		LineCap::Butt => stroke::LineCap::Butt,
		LineCap::Square => stroke::LineCap::Square,
		LineCap::Round => stroke::LineCap::Round,
	};
	let mut miter_limit = 0.;
	let line_join = match path.line_join() {
		LineJoin::Miter(mv) => {
			miter_limit = mv;
			canvas::LineJoin::Miter
		},
		LineJoin::Bevel => canvas::LineJoin::Bevel,
		LineJoin::Round => canvas::LineJoin::Round,
	};
	(line_width, line_cap, line_join, miter_limit)
}