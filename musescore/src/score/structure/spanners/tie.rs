use crate::*;
use crate::score::*;

/// # [Tie](https://en.wikipedia.org/wiki/Tie_(music))
/// a Tie has a Note as startElement/endElement
#[derive(Debug, Clone)]
pub struct Tie {
	element: ElementData,

	spanner_data: SpannerData,
}

impl Element for Tie {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}


#[derive(Debug, Clone)]
pub struct TieSegment {
	element: ElementData,

	segment_data: SpannerSegment,
}

impl Element for TieSegment {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}
