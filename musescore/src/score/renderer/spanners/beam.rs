use crate::score::*;
use crate::drawing::{PainterRef};

pub struct BeamRenderer {}

impl Renderer<Beam> for BeamRenderer {
	fn layout(_e: El<Beam>) {
		unimplemented!()
	}

	fn render(_e: El<Beam>, _state: &mut RendererState, _painter: PainterRef) {
		unimplemented!()
	}
}