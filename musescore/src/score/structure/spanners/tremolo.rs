use crate::score::*;
use crate::LineF;

/// # Tremolo
#[derive(Debug, Clone)]
pub struct Tremolo {
	element: ElementData,

	tremolo_type: TremoloType,
	chord1: Option<El<Chord>>,
	chord2: Option<El<Chord>>,
	duration_type: Duration,
	path: LineF, // Polyline!

	lines: i32,
	tremolo_placement: TremoloPlacement,
}

impl Element for Tremolo {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Tremolo }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TremoloType {
	InvalidTremolo = 0,
	R8 = 1,
	R16 = 2,
	R32 = 3,
	R64 = 4,
	/// one note tremolo (repeat)
	BuzzRoll = 5,
	C8 = 6,
	C16 = 7,
	C32 = 8,
	/// two note tremolo (change)
	C64 = 9,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TremoloPlacement {
	Default = 0,
	StemCenter
}