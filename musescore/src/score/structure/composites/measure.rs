use crate::score::*;

/// One measure in a system
#[derive(Debug, Clone)]
pub struct Measure {
	element: ElementData,

	measure_data: MeasureData,
	segments: OrderedCollecton<El<Segment>>,

	mm_rest: Option<MeasureRefWeak>,

	timesig: Fraction,
	repeat_count: i32,

	no_mode: MeasureNumberMode,
}

impl Measure {
	pub fn segment_iter_range(&self, l: Option<Fraction>, r: Option<Fraction>) -> impl DoubleEndedIterator<Item=&El<Segment>> {
		let ret = match (l, r) {
			(Some(l), Some(r)) => self.segments.range(l.ticks()..r.ticks()),
			(Some(l), None) => self.segments.range(l.ticks()..),
			(None, Some(r)) => self.segments.range(..r.ticks()),
			(None, None) => self.segments.range(..)
		};
		ret.map(|(t, v)| v)
	}

	pub fn segment_next_iter(&self, tick: Fraction) -> impl DoubleEndedIterator<Item=&El<Segment>> {
		self.segment_iter_range(Some(Fraction::from_ticks(tick.ticks())), None)
	}
	pub fn segment_prev_iter(&self, tick: Fraction) -> impl DoubleEndedIterator<Item=&El<Segment>> {
		self.segment_iter_range(None, Some(Fraction::from_ticks(tick.ticks() + 1))).rev()
	}
}

impl Element for Measure {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Measure }
}

impl MeasureTrait for Measure {
	fn measure_data(&self) -> &MeasureData { &self.measure_data }
	fn measure_data_mut(&mut self) -> &mut MeasureData {&mut self.measure_data }
}

#[derive(Debug, Clone, Copy)]
pub enum MeasureNumberMode {
	/// show measure number depending on style
	Auto,
	/// always show measure number
	Show,
	/// don’t show measure number
	Hide
}