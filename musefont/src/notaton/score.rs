use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Score {
	inner: Rc<RefCell<InnerScore>>
}

impl std::fmt::Debug for Score {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Score").finish()
	}
}

impl Score {
	pub fn new() -> Self { Self { inner: Rc::new(RefCell::new(InnerScore::default()))} }

	fn inner(&self) -> &InnerScore { unsafe { &*RefCell::as_ptr(&self.inner) } }
	fn inner_mut(&self) -> &mut InnerScore { unsafe { &mut *RefCell::as_ptr(&self.inner) } }
}

pub struct InnerScore {
}

impl Default for InnerScore {
	fn default() -> Self { Self {
	}}
}