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
			systems: vec![],
			measures: MeasureList::new(),
			parts: vec![],
			staves: vec![],
			style,
			note_head_width
		}))
	}

	fn inner(&self) -> Ref<InnerScore> { self.0.borrow_el() }
	fn inner_mut(&self) -> RefMut<InnerScore> { self.0.borrow_mut_el() }

	pub fn font(&self) -> Ref<ScoreFont> { Ref::map(self.inner(), |r| &r.font) }
	pub fn font_mut(&self) -> RefMut<ScoreFont> { RefMut::map(self.inner_mut(), |r| &mut r.font) }

	pub fn style(&self) -> Ref<Style> { Ref::map(self.inner(), |r| &r.style) }
	pub fn spatium(&self) -> f32 { self.style().value_f32(StyleName::Spatium) }
	pub fn note_head_width(&self) -> f32 { self.inner().note_head_width }
}

impl std::fmt::Debug for Score {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Score").finish()
	}
}

#[derive(Clone)]
pub struct InnerScore {
	font: ScoreFont,

	systems: Vec<El<System>>,
	// Contains a list of all the measures which hold notes and segments
	measures: MeasureList,
	parts: Vec<El<Part>>,
	staves: Vec<El<Staff>>,

	style: Style,
	note_head_width: f32,
}