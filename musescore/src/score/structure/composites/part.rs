use crate::score::*;
use crate::remove_element;

#[derive(Debug, Clone)]
pub struct Part {
	element: ElementData,

	part_name: String,
	instruments: InstrumentList,
	staves: Vec<El<Staff>>,

	/// show part in partitur if true
	show: bool,

	/// User specified color for helping to label parts
	color: u32, // 0x3399ff
}

impl Part {
	pub fn new(score: Score, name: String) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		part_name: name,
		instruments: InstrumentList::new(),
		staves: vec![],
		show: true,
		color: 0x3399FF
	})}

	pub fn staff_count(&self) -> usize { self.staves.len() }

	pub fn staves(&self) -> &Vec<El<Staff>> { &self.staves }

	pub fn insert_staff(&mut self, staff: El<Staff>, mut idx: StaffId) {
		if let ElementRefWeak::Part(part) = self.get_ref() {
			if idx as usize > self.staves.len() {
				idx = self.staves.len() as StaffId;
			}
			self.staves.insert(idx as usize, staff.clone());
			staff.borrow_mut_el().set_part(Some(part))
		}
	}

	pub fn remove_staff(&mut self, staff: &El<Staff>) {
		remove_element(&mut self.staves, staff)
	}
}

impl Element for Part {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Part }

	fn part(&self) -> Option<El<Part>> { self.get_ref_ty::<Part>() }
}

pub type InstrumentList = OrderedCollecton<Instrument>;

#[cfg(test)]
mod tests {
	use crate::testing;
	use crate::score::*;

	#[test]
	fn test_add_staves() {
		let score = testing::setup_score();

		let staff1 = Staff::new(score.clone());
		let staff2 = Staff::new(score.clone());

		let part = Part::new(score.clone(), "Triangle".to_string());
		part.with_mut(|mut part| {
			part.insert_staff(staff1.clone(), 0);
			part.insert_staff(staff2.clone(), 1);
			assert_eq!(part.staff_count(), 2);

			part.remove_staff(&staff1);
			assert_eq!(part.staff_count(), 1);
			part.remove_staff(&staff2);
			assert_eq!(part.staff_count(), 0);
		});
	}
}