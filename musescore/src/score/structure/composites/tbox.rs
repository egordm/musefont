use crate::score::*;

#[derive(Debug, Clone)]
pub struct TBoxData {
	/// Distance from previous system (left border for hbox)
	/// initialized with StyleName::systemFrameDistance
	top_gap: f32,
	/// Distance to next system (right border for hbox)
	/// Initialized with Sid::frameSystemDistance
	bottom_gap: f32,

	/// inner margins in metric mm
	left_margin: f32,
	/// inner margins in metric mm
	right_margin: f32,
	/// inner margins in metric mm
	top_margin: f32,
	/// inner margins in metric mm
	bottom_margin: f32,
}

impl Default for TBoxData {
	fn default() -> Self {Self{
		top_gap: 0.0,
		bottom_gap: 0.0,
		left_margin: 0.0,
		right_margin: 0.0,
		top_margin: 0.0,
		bottom_margin: 0.0
	}}
}

/// vertical frame
#[derive(Debug, Clone)]
pub struct VBox {
	box_data: TBoxData,
	box_height: Spatium,
}

impl MeasureTrait for VBox {

}

/// horizontal frame
#[derive(Debug, Clone)]
pub struct HBox {
	box_data: TBoxData,
	box_width: Spatium,
	create_system_header: bool,
}

impl MeasureTrait for HBox {

}
