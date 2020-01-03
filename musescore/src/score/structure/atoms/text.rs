use crate::score::*;
use crate::{RectF, Point2F};

#[derive(Debug, Clone)]
pub struct Text {
	element: ElementData,

	font_style: FontStyle,
	align: Align,
	frame_type: FrameType,
	family: String,
	size: f32,
	bg_color: Color,
	frame_color: Color,
	frame_width: Spatium,
	padding_width: Spatium,
	frame_round: i32,

	// TODO: Make int a enum
	text: String,
	text_invalid: bool,
	layout: Vec<TextBlock>,
	layout_invalid: bool,

	text_style_id: TStyleId,

	layout_to_parent_width: bool,
	hex_state: i32,
	primed: bool,

	frame: RectF,
}

impl Element for Text {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }
}

/// Represents a block of formatted text
#[derive(Debug, Clone)]
pub struct TextBlock {
	fragments: Vec<TextFragment>,
	y: f32,
	line_spacing: f32,
	bbox: RectF,
	eol: bool,
}

/// Contains a styled text
#[derive(Debug, Clone)]
pub struct TextFragment {
	format: CharFormat,
	/// y is relative to TextBlock->y()
	pos: Point2F,
	text: String,
}

#[derive(Debug, Clone)]
pub struct CharFormat {
	font_style: FontStyle,
	preedit: bool,
	valign: VerticalAlignment,
	font_size: f32,
	font_family: String,
}

impl Default for CharFormat {
	fn default() -> Self {Self {
		font_style: FontStyle::Normal,
		preedit: false,
		valign: VerticalAlignment::AlignNormal,
		font_size: 10.0,
		font_family: String::new()
	}}
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum VerticalAlignment {
	AlignNormal = 0,
	AlignSuperScript = 1,
	AlignSubScript = 2,
}