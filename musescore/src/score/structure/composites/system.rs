use crate::score::*;
use crate::RectF;

/// One row of measures for all instruments;
/// a complete piece of the timeline
#[derive(Debug, Clone)]
pub struct System {
	element: ElementData,

	measures: Vec<MeasureRef>,
	staves: Vec<SysStaff>,
	spanner_segments: Vec<SpannerSegmentRef>,

	/// left margin for instrument name, brackets etc.
	left_margin: f32,
	fixed_down_distance: bool,

	// TODO
}

impl System {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),

		measures: vec![],
		staves: vec![],
		spanner_segments: vec![],
		left_margin: 0.0,
		fixed_down_distance: false
	})}

}

impl Element for System {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::System }
}

#[derive(Debug, Clone)]
pub struct SysStaff {
	bbox: RectF,
}

impl Default for SysStaff {
	fn default() -> Self { Self {
		bbox: Default::default()
	}}
}