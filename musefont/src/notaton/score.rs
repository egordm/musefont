use std::{cell::RefCell, rc::Rc, collections::HashMap};
use crate::*;

pub type ElemId = u32;
pub const ELEMENTID_NONE: ElemId = 0;

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

	/*pub fn add_element<T: ElementTrait>(&mut self, e: T) -> ElementTRef {

	}*/
}

pub struct InnerScore {
	elements: HashMap<ElemId, ElementTRef>,
	element_cursor: ElemId,
}

impl Default for InnerScore {
	fn default() -> Self { Self {
		elements: HashMap::new(),
		element_cursor: ELEMENTID_NONE + 1,
	}}
}