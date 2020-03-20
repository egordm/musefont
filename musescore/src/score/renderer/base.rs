use crate::score::{Element, El};
use crate::drawing::{PainterRef};

pub trait Renderer<T: Element> {
	fn layout(e: El<T>);

	fn render(e: El<T>, state: &mut RendererState, painter: PainterRef);
}

pub struct RendererState {
	debug: bool,
}

impl RendererState {
	pub fn new() -> Self { Self { debug: false } }

	pub fn set_debug(&mut self, v: bool) { self.debug = v }
	pub fn debug(&self) -> bool { self.debug }
}