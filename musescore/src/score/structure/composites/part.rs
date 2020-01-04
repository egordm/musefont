use crate::score::*;

#[derive(Debug, Clone)]
pub struct Part {
	element: ElementData,

	part_name: String,
	instruments: InstrumentList,
	staves: Vec<El<Staff>>,

	/// show part in partitur if true
	show: bool,

	/// User specified color for helping to label parts
	color: i32, // 0x3399ff
}

impl Element for Part {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Part }

	fn part(&self) -> Option<El<Part>> { self.get_ref_ty::<Part>() }
}

pub type InstrumentList = OrderedCollecton<Instrument>;
