use crate::*;
use crate::score::*;
use crate::drawing::*;

pub struct StaffRenderer {

}

impl Renderer<Staff> for StaffRenderer {
	fn layout(e: El<Staff>) {
		unimplemented!()
	}

	fn render(e: El<Staff>, state: &mut RendererState, painter: PainterRef) {
		unimplemented!()
	}
}

impl StaffRenderer {
	fn layout_chords(s: SegmentRef) {

	}
}


// TODO: layout chords 1
// TODO: layout chords 2
// TODO: layout chords 3