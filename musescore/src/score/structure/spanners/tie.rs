use crate::*;
use crate::score::*;

/// # [Tie](https://en.wikipedia.org/wiki/Tie_(music))
/// a Tie has a Note as startElement/endElement
#[derive(Debug, Clone)]
pub struct Tie {
	spanner_data: Spanner,
}

#[derive(Debug, Clone)]
pub struct TieSegment {
	segment_data: SpannerSegment,
}