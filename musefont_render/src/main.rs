extern crate piston_window;

use piston_window::*;
use musefont::*;
use std::path::{PathBuf};

struct PaintContext {
	instructions: Vec<DrawData>,
	pos: Vec2F
}

impl PaintContext {
	pub fn new() -> Self { Self { instructions: Vec::new(), pos: Vec2F::default() }}
}

impl Painter for PaintContext {
	fn draw(&mut self, data: DrawData) {
		self.instructions.push(DrawData::new(data.symid(), data.scale(), data.pos() + self.pos))
	}

	fn translate(&mut self, pt: Vec2F) {
		self.pos += pt;
	}
}

impl PaintContext {
	pub fn render(&self, score: &Score, ctx: &Context, tx_ctx: &mut G2dTextureContext, gfx_ctx: &mut G2d) {
		let point_size = 64.;

		let font = score.font_mut();
		for ins in &self.instructions {
			let img = font.pixmap(ins.symid(), &ins.scale(), 64.,
			            RasterizationOptions::GrayscaleAa, Format::A8).expect("Failed rendering image");

			let tex = Texture::from_memory_alpha(
				tx_ctx, img.pixels(), img.width(), img.height(),
				&TextureSettings::new()
			).unwrap() as G2dTexture;

			let rel_pos = ins.pos() * point_size;
			let pos = ctx.transform.trans(200., 200.).trans(rel_pos.x as f64, rel_pos.y as f64);
			image(&tex, pos, gfx_ctx);
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

	let note_m = Note::new(score.clone());
	let mut note = note_m.borrow_mut();
	note.set_duration(Duration::new(DurationType::Whole, 0));

	let mut window: PistonWindow =
		WindowSettings::new("Musefont Render", [640, 480])
			.exit_on_esc(true).build().unwrap();
	let mut tex_context = window.create_texture_context();


	let mut painter = PaintContext::new();
	note.draw(&mut painter);


	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics, _device| {
			clear([0.; 4], graphics);
			painter.render(&score, &context, &mut tex_context, graphics)
		});

		//std::thread::sleep(std::time::Duration::from_millis(500));
	}
}
