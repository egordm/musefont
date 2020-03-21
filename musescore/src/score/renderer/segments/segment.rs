use crate::score::*;
use crate::drawing::PainterRef;

pub struct SegmentRenderer {}

impl Renderer<Segment> for SegmentRenderer {
	fn layout(e: El<Segment>) {
		e.with(|e| {
			for element in e.elements() {
				if let Some(element) = element {
					ElementRef::layout(element.clone())
				}
			}
		});
	}

	fn render(e: El<Segment>, state: &mut RendererState, painter: PainterRef) {
		e.with(|e| {
			for element in e.elements() {
				if let Some(element) = element {
					ElementRef::render(element.clone(), state, painter)
				}
			}
		});
	}
}
