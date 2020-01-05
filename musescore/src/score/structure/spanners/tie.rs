use crate::*;
use crate::score::*;
use std::convert::TryInto;

/// # [Tie](https://en.wikipedia.org/wiki/Tie_(music))
/// a Tie has a Note as startElement/endElement
#[derive(Debug, Clone)]
pub struct Tie {
	element: ElementData,

	spanner_data: SpannerData,
}

impl Tie {
	pub fn start_note(&self) -> Option<El<Note>> {
		self.start_element().and_then(|e| e.clone().try_into().ok())
	}
	pub fn end_note(&self) -> Option<El<Note>> {
		self.end_element().and_then(|e| e.clone().try_into().ok())
	}
}

impl SpannerTrait for Tie {
	fn spanner_data(&self) -> &SpannerData { &self.spanner_data }
	fn spanner_data_mut(&mut self) -> &mut SpannerData { &mut self.spanner_data}
}

impl Element for Tie {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Tie }

	/*fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p)
			.if_none(|| self.get_spanner_property(p))
			.if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_spanner_property(p, v.clone())
			|| self.set_custom_property(p, v)
	}*/
}


#[derive(Debug, Clone)]
pub struct TieSegment {
	element: ElementData,

	segment_data: SpannerSegment,
}

impl Element for TieSegment {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::TieSegment }
}
