use log::{warn};
use crate::font::*;
use crate::score::*;

#[derive(Clone)]
pub struct Score(El<InnerScore>);

impl Score {
	pub fn new(font: ScoreFont) -> Self {
		let note_head_width = font.width(SymName::NoteheadBlack, 1.); // TODO: spatium / spatium20
		let mut style = Style::new();
		style.precompute_values();
		Self(El::from(InnerScore {
			font,
			systems: vec![],
			measures: OrderedCollecton::new(),
			parts: vec![],
			staves: vec![],
			spanners: OrderedCollecton::new(),
			style,
			note_head_width
		}))
	}

	fn inner(&self) -> Ref<InnerScore> { self.0.borrow_el() }
	fn inner_mut(&self) -> RefMut<InnerScore> { self.0.borrow_mut_el() }

	pub fn font(&self) -> Ref<ScoreFont> { Ref::map(self.inner(), |r| &r.font) }
	pub fn font_mut(&self) -> RefMut<ScoreFont> { RefMut::map(self.inner_mut(), |r| &mut r.font) }

	pub fn style(&self) -> Ref<Style> { Ref::map(self.inner(), |r| &r.style) }
	pub fn spatium(&self) -> f32 { self.style().value_f32(StyleName::Spatium) }
	pub fn point(&self, sp: Spatium) -> f32 { sp.0 * self.spatium() }
	pub fn note_head_width(&self) -> f32 { self.inner().note_head_width }

	pub fn staves(&self) -> Ref<StaffList> { Ref::map(self.inner(), |r| &r.staves) }
	pub fn staff_count(&self) -> usize { self.inner().staves.len() }
	pub fn staff(&self, i: StaffId) -> Option<El<Staff>> { self.inner().staves.get(i as usize).cloned() }

	pub fn add(&mut self, _e: ElementRef) {
		unimplemented!()
	}
}

impl Score {
	/// Adds a part to the score
	/// TODO: Warning: Staves in the part are not added.
	pub fn insert_part(&self, part: El<Part>, idx: StaffId) {
		let mut inserted = false;
		let mut staff = 0;
		{
			let parts = &mut self.inner_mut().parts;
			for i in 0..parts.len() {
				if staff >= idx as usize {
					parts.insert(i, part.clone());
					inserted = true;
					break;
				}
				staff += parts[i].borrow_el().staff_count();
			}
			if !inserted { parts.push(part.clone()) }
		}
		// TODO: instrument changed?
	}

	/// Removes a part from the score
	/// TODO: Warning: Staves attached to the part are not removed.
	pub fn remove_part(&self, part: &El<Part>) {
		remove_element(&mut self.inner_mut().parts, part);
		// TODO: instrument changed?
	}

	/// Inserts a staff into the score and adds it to a corresponding part
	pub fn insert_staff(&self, staff: El<Staff>, part: &El<Part>, rel_idx: StaffId) {
		part.borrow_mut_el().insert_staff(staff.clone(), rel_idx);
		let idx = self.staff_idx(part.clone()) + rel_idx;
		self.inner_mut().staves.insert(idx as usize, staff);
		// TODO: update spenners their tracks
	}

	/// Removes a staff from the score and the corresponding part
	pub fn remove_staff(&self, staff: &El<Staff>) {
		if let Some(part) = staff.borrow_el().part().upgrade().clone() {
			let idx = staff.borrow_el().staff_id();
			remove_element(&mut self.inner_mut().staves, &staff);
			part.borrow_mut_el().remove_staff(staff);
			// TODO: update spenners their tracks
		} else {
			warn!("Tried to remove a staff which has no part assigned.");
			// TODO: mayby return a result instead?
		}
	}

	pub fn staff_idx(&self, part: El<Part>) -> StaffId {
		let mut idx = 0;
		for p in self.inner().parts.iter() {
			if part == *p { break }
			else { idx += p.borrow_el().staff_count() }
		}
		return idx as StaffId;
	}
}

impl std::fmt::Debug for Score {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Score").finish()
	}
}

#[derive(Clone)]
pub struct InnerScore {
	font: ScoreFont,

	systems: Vec<El<System>>,
	// Contains a list of all the measures which hold notes and segments
	measures: OrderedCollecton<MeasureRef>,
	parts: PartList,
	staves: StaffList,
	spanners: OrderedCollecton<SpannerRef>,

	style: Style,
	note_head_width: f32,
}

pub type StaffList = Vec<El<Staff>>;
pub type PartList = Vec<El<Part>>;

#[cfg(test)]
mod tests {
	use crate::testing;
	use crate::score::*;

	#[test]
	fn test_add_remove_staves() {
		let score = testing::setup_score();

		let staves = [
			Staff::new(score.clone()),
			Staff::new(score.clone()),
			Staff::new(score.clone())
		];
		let staff_extra = Staff::new(score.clone());

		let part = Part::new(score.clone(), "Triangle".to_string());
		score.insert_part(part.clone(), 0);

		for (i, staff) in staves.iter().enumerate() {
			score.insert_staff(staff.clone(), &part, i as StaffId);
		}
		score.insert_staff(staff_extra.clone(), &part, 0);

		assert_eq!(score.staff_count(), 4);
		part.with(|part| {
			assert_eq!(part.staff_count(), 4);
			assert_eq!(part.staves()[0], staff_extra)
		});

		for (i, staff) in staves.iter().enumerate() {
			score.remove_staff(staff);
		}
		assert_eq!(score.staff_count(), 1);
		part.with(|part| {
			assert_eq!(part.staff_count(), 1);
			assert_eq!(part.staves()[0], staff_extra)
		});
	}
}