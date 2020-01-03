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
pub struct Spanner {
	/// the element the spanner end is anchored to (read-only)
	start_element: Option<ElementRef>,
	/// the element the spanner start is anchored to (read-only)
	end_element: Option<ElementRef>,

	anchor: Anchor,
	/// tick start position
	tick: Fraction,
	ticks: Fraction,
	/// tick end position
	tick2: i32,
	broken: bool,

	segments: Vec<SpannerSegmentRef>,
	segment_pool: Vec<SpannerSegmentRef>,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Anchor {
	Segment = 0,
	Measure = 1,
	Chord = 2,
	Note = 3,
}