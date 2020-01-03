use crate::score::*;

#[derive(Debug, Clone)]
pub struct Line {
	line_width: f32,
	line_color: Color,
	line_style: LineStyle,
	dash_line_len: f32,
	dash_gap_len: f32,
	diagonal: bool,
}

#[derive(Debug, Clone)]
pub struct LineSegment {
	segment_data: SpannerSegment,
}