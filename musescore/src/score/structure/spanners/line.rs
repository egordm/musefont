use crate::score::*;

#[derive(Debug, Clone)]
pub struct LineSpanner {
	element: ElementData,

	line_width: f32,
	line_color: Color,
	line_style: LineStyle,
	dash_line_len: f32,
	dash_gap_len: f32,
	diagonal: bool,
}

impl Element for LineSpanner {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::LineSpanner }
}

#[derive(Debug, Clone)]
pub struct LineSegment {
	element: ElementData,

	segment_data: SpannerSegment,
}

impl Element for LineSegment {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::LineSegment }
}