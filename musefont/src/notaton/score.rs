use std::{cell::RefCell, rc::Rc};
use crate::ScoreFont;

#[derive(Clone)]
pub struct Score(Rc<RefCell<InnerScore>>);

impl std::fmt::Debug for Score {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Score").finish()
	}
}

impl Score {
	pub fn new(font: ScoreFont) -> Self {
		Self(Rc::new(RefCell::new(InnerScore { font })))
	}

	fn inner(&self) -> &InnerScore { unsafe { &*RefCell::as_ptr(&self.0) } }
	fn inner_mut(&self) -> &mut InnerScore { unsafe { &mut *RefCell::as_ptr(&self.0) } }

	pub fn font(&self) -> &ScoreFont { &self.inner().font }
	pub fn font_mut(&self) -> &mut ScoreFont { &mut self.inner_mut().font }
}

pub struct InnerScore {
	font: ScoreFont,
}