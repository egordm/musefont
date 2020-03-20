use crate::score::*;
use crate::drawing::{PainterRef};

pub struct MeasureRenderer {}


impl Renderer<Measure> for StemRenderer {
	fn layout(e: El<Measure>) {
		unimplemented!()
	}

	fn render(e: El<Measure>, state: &mut RendererState, painter: PainterRef) {
		e.with(|e| {
			//e.segments().
		});
	}
}
