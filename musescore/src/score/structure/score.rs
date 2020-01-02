use crate::font::*;
use crate::score::*;

#[derive(Clone)]
pub struct Score(El<InnerScore>);

impl Score {
	pub fn new(font: ScoreFont) -> Self {
		let note_head_width = font.width(SymName::NoteheadBlack, 1.); // TODO: spatium / spatium20
		let mut style = Style::new();
		style.precompute_values();
		Self(El::from(InnerScore {
			font,
			parts: vec![],
			staves: vec![],
			style,
			note_head_width
		}))
	}
}

impl std::fmt::Debug for Score {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Score").finish()
	}
}

#[derive(Clone)]
pub struct InnerScore {
	font: ScoreFont,

	// measures: MeasureBaseList,   here are the notes
	parts: Vec<El<Part>>,
	staves: Vec<El<Staff>>,

	style: Style,
	note_head_width: f32,
}