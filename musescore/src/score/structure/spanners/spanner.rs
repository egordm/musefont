use crate::*;
use crate::score::*;

/// # SpannerSegment
/// parent: System
#[derive(Debug, Clone)]
pub struct SpannerSegment {
	spanner: Option<SpannerRefWeak>,
	spanner_type: SpannerType,

	p2: Point2F,
	offset2: Point2F,
}

/// Data for slurs, ties, lines etc
#[derive(Debug, Clone)]
pub struct SpannerData {
	/// the element the spanner end is anchored to (read-only)
	start_element: Option<ElementRef>,
	/// the element the spanner start is anchored to (read-only)
	end_element: Option<ElementRef>,

	anchor: Anchor,
	/// tick start position
	tick: Fraction,
	ticks: Fraction,
	/// tick end position
	track2: i32,
	broken: bool,

	segments: Vec<SpannerSegmentRef>,
	segment_pool: Vec<SpannerSegmentRef>,
}

pub trait SpannerTrait: Element {
	fn spanner_data(&self) -> &SpannerData;
	fn spanner_data_mut(&mut self) -> &mut SpannerData;

	fn start_element(&self) -> Option<&ElementRef> { self.spanner_data().start_element.as_ref() }
	fn set_start_element(&mut self, v: Option<ElementRef>) { self.spanner_data_mut().start_element = v }
	fn end_element(&self) -> Option<&ElementRef> { self.spanner_data().end_element.as_ref() }
	fn set_end_element(&mut self, v: Option<ElementRef>) { self.spanner_data_mut().end_element = v }
	
	fn anchor(&self) -> Anchor { self.spanner_data().anchor }
	fn set_anchor(&mut self, v: Anchor) { self.spanner_data_mut().anchor = v }
	
	fn tick(&self) -> Fraction { self.spanner_data().tick }
	fn set_tick(&mut self, v: Fraction) { self.spanner_data_mut().tick = v }
	fn ticks(&self) -> Fraction { self.spanner_data().ticks }
	fn set_ticks(&mut self, v: Fraction) { self.spanner_data_mut().ticks = v }
	fn track2(&self) -> i32 { self.spanner_data().track2 }
	fn set_track2(&mut self, v: i32) { self.spanner_data_mut().track2 = v }
	
	fn broken(&self) -> bool { self.spanner_data().broken }
	fn set_broken(&mut self, v: bool) { self.spanner_data_mut().broken = v }
	
	fn segments(&self) -> &Vec<SpannerSegmentRef> { &self.spanner_data().segments }
	fn set_segments(&mut self, v: Vec<SpannerSegmentRef>) { self.spanner_data_mut().segments = v }
	fn segment_pool(&self) -> &Vec<SpannerSegmentRef> { &self.spanner_data().segment_pool }
	fn set_segment_pool(&mut self, v: Vec<SpannerSegmentRef>) { self.spanner_data_mut().segment_pool = v }

	fn get_spanner_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::SpannerTick => SpannerTrait::tick(self).ticks().into(),
			PropertyId::SpannerTicks => self.ticks().ticks().into(),
			PropertyId::SpannerTrack2 => self.track2().into(),
			PropertyId::Anchor => ValueVariant::from_enum(self.anchor()),
			PropertyId::LocationStaves => ((self.track2() / constants::VOICES as i32) - (self.track() / constants::VOICES as i32)).into(),
			PropertyId::LocationVoices => ((self.track2() % constants::VOICES as i32) - (self.track() / constants::VOICES as i32)).into(),
			PropertyId::LocationFractions => self.ticks().ticks().into(),
			// TODO: location property
			_ => ValueVariant::None
		}
	}
	fn set_spanner_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::SpannerTick => v.with_value(|v| {
				self.set_tick(Fraction::from_ticks(v));
				self.set_start_element(None);
				self.set_end_element(None);
				// TODO: remove spanner
			}),
			PropertyId::SpannerTicks => v.with_value(|v| {
				self.set_ticks(Fraction::from_ticks(v));
				self.set_end_element(None);
			}),
			PropertyId::Track => v.with_value(|v| {
				self.set_ticks(Fraction::from_ticks(v));
				self.set_start_element(None);
			}),
			PropertyId::SpannerTrack2 => v.with_value(|v| {
				self.set_track2(v);
				self.set_end_element(None);
			}),
			PropertyId::Anchor => v.with_enum(|v| self.set_anchor(v)),
			_ => false,
		}
	}
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Anchor {
	Segment = 0,
	Measure = 1,
	Chord = 2,
	Note = 3,
}