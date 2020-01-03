use crate::score::*;
use crate::font::SymName;

#[derive(Debug, Clone)]
pub struct RestData {
	pub(super) elements: Vec<ElementRef>,
	pub(super) duration_type: Duration,
	/// -1, 0, +1, used for crossbeaming
	pub(super) staff_move: i32,

	// TODO: tab duraiton symbol && lyrics

	pub(super) beam: Option<ElWeak<Beam>>,
	pub(super) beam_mode: BeamMode,

	/// actual stem direction
	pub(super) up: bool,
	pub(super) small: bool,

	/// CrossMeasure: combine 2 tied notes if across a bar line and can be combined in a single duration
	/// 0: no cross-measure modification; 1: 1st note of a mod.; -1: 2nd note
	pub(super) cross_measure: CrossMeasure,
	/// the total Duration type of the combined notes
	pub(super) cross_measure_tdur: Duration,
}

#[derive(Debug, Clone)]
pub struct Rest {
	element: ElementData,
	duration_data: DurationElementData,
	rest_data: RestData,
	segment_data: SegmentData,

	sym: SymName,
	/// depends on rest symbol
	dotline: i32,
	/// width of multi measure rest
	mm_width: f32,
	/// invisible and not selectable for user
	gap: bool,
	dots: Vec<El<NoteDot>>
}

impl Element for Rest {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}

impl DurationElement for Rest {
	fn duration_data(&self) -> &DurationElementData { &self.duration_data }
	fn duration_data_mut(&mut self) -> &mut DurationElementData { &mut self.duration_data }
}

impl SegmentTrait for Rest {
	fn segment_data(&self) -> &SegmentData { &self.segment_data }
	fn segment_data_mut(&mut self) -> &mut SegmentData { &mut self.segment_data }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CrossMeasure {
	Unknown = -1,
	None = 0,
	First = 1,
	Second = 2
}