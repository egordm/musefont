use crate::score::*;

#[derive(Debug, Clone)]
pub struct Part {
	part_name: String,
	instruments: InstrumentList,
	staves: Vec<El<Staff>>,

	/// show part in partitur if true
	show: bool,

	/// User specified color for helping to label parts
	color: i32, // 0x3399ff
}

pub type InstrumentList = OrderedCollecton<Instrument>;
