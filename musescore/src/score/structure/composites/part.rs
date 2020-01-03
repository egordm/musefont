use crate::score::*;

#[derive(Debug, Clone)]
pub struct Part {
	part_name: String,
	instruments: InstrumentList,
	staves: Vec<El<Staff>>,

	show: bool,
}

pub type InstrumentList = OrderedCollecton<Instrument>;
