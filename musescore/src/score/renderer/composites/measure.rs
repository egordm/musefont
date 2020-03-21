use crate::score::*;
use crate::drawing::{PainterRef};

pub struct MeasureRenderer {}


impl Renderer<Measure> for MeasureRenderer {
	fn layout(e: El<Measure>) {
		e.with(|e| {
			for segment in e.segments().iter_vals() {
				SegmentRenderer::layout(segment.clone())
			}
		});
	}

	fn render(e: El<Measure>, state: &mut RendererState, painter: PainterRef) {
		e.with(|e| {
			for segment in e.segments().iter_vals() {
				SegmentRenderer::render(segment.clone(), state, painter)
			}
		});
	}
}
