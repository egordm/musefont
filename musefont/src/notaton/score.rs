use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};
use generational_arena::{Arena, Index};
use crate::*;

pub type ElemId = Index;

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

	fn inner(&self) -> Ref<InnerScore> { RefCell::borrow(&self.inner) }
	fn inner_mut(&self) -> RefMut<InnerScore> { RefCell::borrow_mut(&self.inner) }

	pub fn add_element<T: ElementTrait>(&mut self, e: T) -> Option<ElementRef> {
		let id = self.inner_mut().elements.insert(e.into_ref()?);
		let mut ret = self.get_element(id)?;
		ret.attach(Some((self.clone(), id)));
		Some(ret)
	}

	pub fn get_element(&self, id: ElemId) -> Option<ElementRef> {
		self.inner().elements.get(id).cloned()
	}
}

pub struct InnerScore {
	elements: Arena<ElementRef>,
}

impl Default for InnerScore {
	fn default() -> Self { Self {
		elements: Arena::new(),
	}}
}