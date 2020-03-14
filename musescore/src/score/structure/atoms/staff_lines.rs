use crate::*;
use crate::score::*;
use std::convert::TryFrom;

/// # StaffLines
/// The StaffLines class is the graphic representation of a staff
/// it draws the horizontal staff lines.
#[derive(Debug, Clone)]
pub struct StaffLines {
	element: ElementData,

	lines: Vec<LineF>,
	lw: f32,
}

impl StaffLines {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		lines: vec![],
		lw: 0.0
	})}

	pub fn lines(&self) -> &Vec<LineF> { &self.lines }
	pub fn y1(&self) -> f32 {
		self.staff().map(|e| e.borrow_el().y()).unwrap_or(0.) + self.ipos().y
	}

	pub fn page_pos(&self) -> Point2F {
		if let (Some(measure), Some(staff)) = (self.measure(), self.staff()) {
			if let Some(system) = measure.as_trait().system() {
				return Point2F::new(
					measure.as_trait().x() + system.borrow_el().x(),
					staff.borrow_el().y() + system.borrow_el().y(),
				);
			}
		}
		return Point2F::default();
	}

	pub fn canvas_pos(&self) -> Point2F {
		let p = self.page_pos();
		// TODO add page pos
		p
	}

}

impl AtomTrait for StaffLines {
	fn chord(&self) -> Option<El<Chord>> { None }
	fn chord_rest(&self) -> Option<ChordRef> {
		None
	}
	fn segment(&self) -> Option<El<Segment>> { None }
	fn measure(&self) -> Option<MeasureRef> {
		self.parent().and_then(|e| MeasureRef::try_from(e).ok())
	}
}

impl Element for StaffLines {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::StaffLines }
}