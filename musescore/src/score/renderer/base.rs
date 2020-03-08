use crate::score::{Element, El};
use crate::drawing::PainterRef;

pub trait Renderer<T: Element> {
	fn layout(e: El<T>);

	fn render(e: El<T>, state: &mut RendererState, painter: PainterRef);
}

pub struct RendererState {

}

impl RendererState {
	pub fn new() -> Self { Self {} }
}