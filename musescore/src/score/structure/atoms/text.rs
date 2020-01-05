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

// TODO: do more
impl Text {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		font_style: FontStyle::NORMAL,
		align: Align::LEFT,
		frame_type: FrameType::NoFrame,
		family: "FreeSerif".to_string(),
		size: 10.0,
		bg_color: [255, 255, 255, 0],
		frame_color: [0, 0, 0, 255],
		frame_width: 0.1,
		padding_width: 0.2,
		frame_round: 0,
		text: String::new(),
		text_invalid: true,
		layout: vec![],
		layout_invalid: true,
		text_style_id: TStyleId::Default,
		layout_to_parent_width: false,
		hex_state: 0,
		primed: false,
		frame: Default::default()
	})}

	pub fn bold(&self) -> bool { self.font_style.bits() & FontStyle::BOLD.bits() > 0  }
	pub fn italic(&self) -> bool { self.font_style.bits() & FontStyle::ITALIC.bits() > 0  }
	pub fn underline(&self) -> bool { self.font_style.bits() & FontStyle::UNDERLINE.bits() > 0  }
	pub fn set_bold(&mut self, v: bool) { self.font_style.set(FontStyle::BOLD, v)  }
	pub fn set_italic(&mut self, v: bool) { self.font_style.set(FontStyle::ITALIC, v)  }
	pub fn set_underline(&mut self, v: bool) { self.font_style.set(FontStyle::UNDERLINE, v)  }

	pub fn align(&self) -> Align { self.align }
	pub fn set_align(&mut self, v: Align) { self.align = v }

	pub fn frame_type(&self) -> FrameType { self.frame_type }
	pub fn set_frame_type(&mut self, v: FrameType) { self.frame_type = v }

	pub fn family(&self) -> &String { &self.family }
	pub fn set_family(&mut self, v: String) { self.family = v }

	pub fn size(&self) -> f32 { self.size }
	pub fn set_size(&mut self, v: f32) { self.size = v }

	pub fn bg_color(&self) -> &Color { &self.bg_color }
	pub fn set_bg_color(&mut self, v: Color) { self.bg_color = v }

	pub fn frame_color(&self) -> &Color { &self.frame_color }
	pub fn set_frame_color(&mut self, v: Color) { self.frame_color = v }
	pub fn frame_width(&self) -> Spatium { self.frame_width }
	pub fn set_frame_width(&mut self, v: Spatium) { self.frame_width = v }

	pub fn padding_width(&self) -> Spatium { self.padding_width }
	pub fn set_padding_width(&mut self, v: Spatium) { self.padding_width = v }
	pub fn frame_round(&self) -> i32 { self.frame_round }
	pub fn set_frame_round(&mut self, v: i32) { self.frame_round = v }

	pub fn text(&self) -> &String { &self.text }
	pub fn set_text(&mut self, v: String) { self.text = v }
	pub fn text_invalid(&self) -> bool { self.text_invalid }
	pub fn set_text_invalid(&mut self, v: bool) { self.text_invalid = v }

	pub fn layout(&self) -> &Vec<TextBlock> { &self.layout }
	pub fn set_layout(&mut self, v: Vec<TextBlock>) { self.layout = v }
	pub fn layout_invalid(&self) -> bool { self.layout_invalid }
	pub fn set_layout_invalid(&mut self, v: bool) { self.layout_invalid = v }

	pub fn text_style_id(&self) -> TStyleId { self.text_style_id }
	pub fn set_text_style_id(&mut self, v: TStyleId) { self.text_style_id = v }

	pub fn layout_to_parent_width(&self) -> bool { self.layout_to_parent_width }
	pub fn set_layout_to_parent_width(&mut self, v: bool) { self.layout_to_parent_width = v }
	pub fn hex_state(&self) -> i32 { self.hex_state }
	pub fn set_hex_state(&mut self, v: i32) { self.hex_state = v }

	pub fn primed(&self) -> bool { self.primed }
	pub fn set_primed(&mut self, v: bool) { self.primed = v }
	pub fn frame(&self) -> &RectF { &self.frame }
	pub fn set_frame(&mut self, v: RectF) { self.frame = v }
}

impl Element for Text {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Text }
}

impl AtomTrait for Text {

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
		font_style: FontStyle::NORMAL,
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