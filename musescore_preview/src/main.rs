use cgmath;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, LineCap, LineJoin};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
	image2_nearest: graphics::Image,
	meshes: Vec<graphics::Mesh>,
}

impl MainState {
	/// Load images and create meshes.
	fn new(ctx: &mut Context) -> GameResult<MainState> {
		let mut image2_nearest = graphics::Image::new(ctx, "/shot.png")?;
		image2_nearest.set_filter(graphics::FilterMode::Nearest);

		let meshes = vec![build_mesh(ctx)?];
		let s = MainState {
			image2_nearest,
			meshes,
		};

		Ok(s)
	}
}

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
	let mb = &mut graphics::MeshBuilder::new();

	mb.polyline(
		DrawMode::Stroke(graphics::StrokeOptions::default()
			.with_line_cap(LineCap::Round)
			.with_line_join(LineJoin::Round)
			.with_line_width(4.)),
		&[
			Point2::new(200.0, 200.0),
			Point2::new(400.0, 200.0),
		],
		Color::new(1.0, 0.0, 0.0, 1.0),
	)?;

	mb.build(ctx)
}

impl event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

		// Draw an image with some options, and different filter modes.
		graphics::draw(
			ctx,
			&self.image2_nearest,
			graphics::DrawParam::new()
				.dest(cgmath::Point2::new(400.0, 400.0))
				.scale(cgmath::Vector2::new(10.0, 10.0)),
		)?;

		// Draw some pre-made meshes
		for m in &self.meshes {
			graphics::draw(ctx, m, DrawParam::new())?;
		}

		// Finished drawing, show it all on the screen!
		graphics::present(ctx)?;
		Ok(())
	}
}

pub fn main() -> GameResult {
	let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		path.push("resources");
		path
	} else {
		path::PathBuf::from("./resources")
	};

	let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

	let (ctx, events_loop) = &mut cb.build()?;

	println!("{}", graphics::renderer_info(ctx)?);
	let state = &mut MainState::new(ctx).unwrap();
	event::run(ctx, events_loop, state)
}