use crate::painter::PfPainter;
use glutin::{ContextBuilder, EventsLoop, WindowBuilder, GlRequest, GlProfile, WindowEvent, Event, VirtualKeyCode, ControlFlow, KeyboardInput};
use pathfinder_geometry::vector::Vector2I;
use pathfinder_canvas::{CanvasRenderingContext2D, CanvasFontContext};
use glutin::dpi::PhysicalSize;
use pathfinder_gpu::resources::FilesystemResourceLoader;
use pathfinder_renderer::gpu::{renderer::Renderer, options::{DestFramebuffer, RendererOptions}};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_color::ColorF;
use pathfinder_renderer::{concurrent::{scene_proxy::SceneProxy, rayon::RayonExecutor}, options::BuildOptions};
use std::path::{PathBuf};


pub type Canvas = CanvasRenderingContext2D;

pub fn start_window<D: FnMut(&mut PfPainter)>(size: Vector2I, name: &str, mut draw_fn: D) {
	let mut event_loop = EventsLoop::new();
	let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor();
	let physical_window_size = PhysicalSize::new(size.x() as f64, size.y() as f64);
	let logical_window_size = physical_window_size.to_logical(hidpi_factor);

	// Open a window.
	let window_builder = WindowBuilder::new().with_title(name)
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
		DestFramebuffer::full_window(size),
		RendererOptions { background_color: Some(ColorF::white()) }
	);

	// Make a canvas. We're going to draw a house.
	let mut canvas = CanvasRenderingContext2D::new(
		CanvasFontContext::from_system_source(), size.to_f32());

	// Draw!
	{
		let mut painter = PfPainter::new(&mut canvas);
		draw_fn(&mut painter);
	}

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