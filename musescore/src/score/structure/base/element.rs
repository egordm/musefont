use crate::*;
use crate::score::*;

pub type Track = i32;

#[derive(Clone, Debug)]
pub struct ElementData {
	score_element: ScoreElementData,
	bbox: RectF,
	scale: f32,
	pos: Point2F,
	offset: Point2F,
	min_dist: f32,
	track: Track,
    flags: ElementFlags,
}

pub trait Element {
}

bitflags! { struct ElementFlags: u32 {
	const NOTHING                = 0x00000000;
	const DROP_TARGET            = 0x00000001;
	const NOT_SELECTABLE         = 0x00000002;
	const MOVABLE                = 0x00000004;
	const COMPOSITION            = 0x00000008;       // true if element is part of another element
	const HAS_TAG                = 0x00000010;       // true if this is a layered element
	const ON_STAFF               = 0x00000020;
	const SELECTED               = 0x00000040;
	const GENERATED              = 0x00000080;
	const INVISIBLE              = 0x00000100;
	const NO_AUTOPLACE           = 0x00000200;
	const SYSTEM                 = 0x00000400;
	const PLACE_ABOVE            = 0x00000800;
	const SIZE_SPATIUM_DEPENDENT = 0x00001000;

	// measure flags
	const REPEAT_END             = 0x00002000;
	const REPEAT_START           = 0x00004000;
	const REPEAT_JUMP            = 0x00008000;
	const IRREGULAR              = 0x00010000;
	const LINE_BREAK             = 0x00020000;
	const PAGE_BREAK             = 0x00040000;
	const SECTION_BREAK          = 0x00080000;
	const NO_BREAK               = 0x00100000;
	const HEADER                 = 0x00200000;
	const TRAILER                = 0x00400000;    // also used in segment
	const KEYSIG                 = 0x00800000;

	// segment flags
	const ENABLED                = 0x01000000;    // used for segments
	const EMPTY                  = 0x02000000;
	const WRITTEN                = 0x04000000;
}}