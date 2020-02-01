use glutin::{ContextBuilder, EventsLoop, WindowBuilder, GlRequest, GlProfile, dpi::PhysicalSize, Event, WindowEvent, KeyboardInput, ControlFlow, VirtualKeyCode};
use pathfinder_geometry::vector::{Vector2I, Vector2F};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_gpu::resources::{FilesystemResourceLoader, ResourceLoader};
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_color::ColorF;
use pathfinder_canvas::{CanvasRenderingContext2D, CanvasFontContext, Path2D, LineJoin};
use pathfinder_content::stroke::{LineCap};
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::options::BuildOptions;

use musescore::*;
use std::path::{PathBuf};
use musescore::font::{ScoreFont, FontMapping, SymName};

pub fn main() {
	let mut event_loop = EventsLoop::new();
	let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor();
	let window_size = Vector2I::new(640, 480);
	let physical_window_size = PhysicalSize::new(window_size.x() as f64, window_size.y() as f64);
	let logical_window_size = physical_window_size.to_logical(hidpi_factor);

	// Open a window.
	let window_builder = WindowBuilder::new().with_title("Musescore Preview")
		.with_dimensions(logical_window_size);

	// Create an OpenGL 3.x context for Pathfinder to use.
	let gl_context = ContextBuilder::new().with_gl(GlRequest::Latest)
		.with_gl_profile(GlProfile::Core)
		.build_windowed(window_builder, &event_loop)
		.unwrap();

	// Load OpenGL, and make the context current.
	let gl_context = unsafe { gl_context.make_current().unwrap() };
	gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

	let resource_loader = FilesystemResourceLoader {
		directory: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources")
	};

	// Create a Pathfinder renderer.
	let mut renderer = Renderer::new(
		GLDevice::new(GLVersion::GL3, 0),
		&resource_loader,
		DestFramebuffer::full_window(window_size),
		RendererOptions { background_color: Some(ColorF::white()) }
	);

	// Make a canvas. We're going to draw a house.
	let mut canvas = CanvasRenderingContext2D::new(
		CanvasFontContext::from_system_source(), window_size.to_f32());

	canvas.set_line_width(3.0);
	canvas.set_line_cap(LineCap::Round);
	canvas.set_line_join(LineJoin::Round);

	let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/gootville");
	let filename = "gootville.otf";
	let config = FontMapping::load(&config).unwrap();
	let font = font::load(&path, filename, &config).expect("Font must load!");

	canvas.set_font(font.font().clone());
	let sym = font.sym(SymName::NoteheadBlack);
	let sym_char = sym.get_char().expect("Should be a valid character");
	canvas.set_font_size(64.);
	canvas.fill_text(&sym_char.to_string(), Vector2F::new(50.0, 50.0));
	//sym.code()

	let mut path = Path2D::new();
	path.move_to(Vector2F::new(50.0, 140.0));
	path.line_to(Vector2F::new(150.0, 60.0));
	path.line_to(Vector2F::new(250.0, 140.0));
	path.close_path();
	canvas.stroke_path(path);

	let mut path = Path2D::new();
	path.move_to(Vector2F::new(50.0, 240.0));
	path.quadratic_curve_to(Vector2F::new(100., 290.), Vector2F::new(150.0, 240.0));
	canvas.stroke_path(path);

	// Render the canvas to screen.
	let scene = SceneProxy::from_scene(canvas.into_scene(), RayonExecutor);
	scene.build_and_render(&mut renderer, BuildOptions::default());
	gl_context.swap_buffers().unwrap();

	// Wait for a keypress.
	event_loop.run_forever(|event| {
		match event {
			Event::WindowEvent { event: WindowEvent::CloseRequested, .. } |
			Event::WindowEvent {
				event: WindowEvent::KeyboardInput {
					input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. },
					..
				},
				..
			} => ControlFlow::Break,
			_ => ControlFlow::Continue,
		}
	})
}