extern crate piston_window;

use piston_window::*;
use musefont::*;
use std::path::{PathBuf};

struct PaintContext {
	instructions: Vec<DrawIns>,
	pos: Vec2F
}

impl PaintContext {
	pub fn new() -> Self { Self { instructions: Vec::new(), pos: Vec2F::default() }}
}

impl Painter for PaintContext {
	fn draw(&mut self, data: DrawIns) {
		let scale_global = Size2F::new(1., -1.);

		self.instructions.push(match data {
			DrawIns::Symbol(id, scale, pos) => {
				DrawIns::Symbol(id, scale, scale_pos(pos + self.pos, scale_global))
			},
			DrawIns::Line(line, width) => {
				DrawIns::Line((line + self.pos) * scale_global, width)
			},
		});
	}

	fn translate(&mut self, pt: Vec2F) {
		self.pos += pt;
	}
}

impl PaintContext {
	pub fn render(&self, score: &Score, ctx: &Context, tx_ctx: &mut G2dTextureContext, gfx_ctx: &mut G2d) {
		let font = score.font_mut();
		let matrix: math::Matrix2d = ctx.transform.trans(0., 480.0);//.scale(1., -1.);//;
		for ins in self.instructions.iter().cloned() {
			match ins {
				DrawIns::Symbol(symid, scale, pos) => {
					let img = font.pixmap(symid, &scale, RasterizationOptions::GrayscaleAa,
					                      Format::A8).expect("Failed rendering image");

					let tex = Texture::from_memory_alpha(
						tx_ctx, img.pixels(), img.width(), img.height(),
						&TextureSettings::new()
					).unwrap() as G2dTexture;

					let rel_pos = pos;
					let pos = matrix.trans(rel_pos.x as f64, rel_pos.y as f64);
					image(&tex, pos, gfx_ctx);
				},
				DrawIns::Line(line, width) => {
					let line = line;
					let p1 = [line.p1.x as f64, line.p1.y as f64];
					let p2 = [line.p2.x as f64, line.p2.y as f64];
					line_from_to([1., 1., 1., 1.], (width) as f64, p1, p2, matrix, gfx_ctx);
				},
			}
		}
	}
}

fn main() {
	let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/gootville");
	let filename = "gootville.otf";
	let config = FontConfig::new(&config).unwrap();
	let font = load_font(&path, filename, &config).unwrap();
	let score = Score::new(font);

	let mut chord = Chord::new(score.clone());
	chord.borrow_mut().set_duration(Duration::new(DurationType::D128th, 0));
	chord.set_pos(Point2F::new(100., 100.5));
	{
		let note = Note::new(score.clone());
		note.borrow_mut().set_duration(Duration::new(DurationType::D128th, 0));
		chord.borrow_mut().add_note(note);
	}

	let mut window: PistonWindow =
		WindowSettings::new("Musefont Render", [640, 480])
			.exit_on_esc(true).build().unwrap();
	let mut tex_context = window.create_texture_context();

	let mut painter = PaintContext::new();
	chord.borrow_mut().layout();
	chord.draw(&mut painter);

	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics, _device| {
			clear([0.; 4], graphics);
			painter.render(&score, &context, &mut tex_context, graphics)
		});

		//std::thread::sleep(std::time::Duration::from_millis(500));
	}
}
