use crate::score::*;
use crate::drawing::PainterRef;

pub struct SegmentRenderer {}

impl Renderer<Segment> for SegmentRenderer {
	fn layout(e: El<Segment>) {
		e.with(|e| {
			for element in e.elements() {
				if let Some(element) = element {
					SegmentRef::layout(element.clone())
				}
			}
		});
	}

	fn render(e: El<Segment>, state: &mut RendererState, painter: PainterRef) {
		e.with(|e| {
			for element in e.elements() {
				if let Some(element) = element {
					SegmentRef::render(element.clone(), state, painter)
				}
			}
		});
	}
}
