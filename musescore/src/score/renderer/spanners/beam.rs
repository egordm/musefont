use crate::score::*;
use crate::drawing::{PainterRef};

pub struct BeamRenderer {}

impl Renderer<Beam> for BeamRenderer {
	fn layout(e: El<Beam>) {
		unimplemented!()
	}

	fn render(e: El<Beam>, state: &mut RendererState, painter: PainterRef) {
		unimplemented!()
	}
}