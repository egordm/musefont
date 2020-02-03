use crate::score::{Element, El};
use crate::drawing::PainterRef;

pub trait Renderer<T: Element> {
	fn layout(e: El<T>);

	fn render(e: El<T>, state: &mut RendererState, painter: &mut PainterRef);
}

pub struct RendererState {

}