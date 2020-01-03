use crate::LineF;

#[derive(Debug, Clone)]
pub struct Stem {
	line: LineF,
	line_width: f32,
	user_len: f32,
	len: f32,
}