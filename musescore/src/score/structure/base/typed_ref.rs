use std::{rc::{Weak, Rc}, cell::{RefCell, Ref, RefMut}};
use super::*;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct ElWeak<T>(Weak<RefCell<T>>);

impl<T> ElWeak<T> {
	pub fn upgrade(&self) -> Option<El<T>> {
		Weak::upgrade(&self.0).map(El)
	}
}

impl<T> Eq for ElWeak<T> {}
impl<T> PartialEq for ElWeak<T> {
	fn eq(&self, other: &Self) -> bool { Weak::ptr_eq(&self.0, &other.0) }
}
impl<T> PartialEq<El<T>> for ElWeak<T> {
	fn eq(&self, other: &El<T>) -> bool { self == &other.downgrade()}
}

impl<T> std::fmt::Debug for ElWeak<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.upgrade() {
			None => write!(f, "ElWeak(None)"),
			Some(_) => write!(f, "ElWeak(Some)"),
		}
	}
}

#[derive(Clone)]
pub struct El<T>(Rc<RefCell<T>>);

impl<T> El<T> {
	pub fn downgrade(&self) -> ElWeak<T> { ElWeak(Rc::downgrade(&self.0)) }

	pub fn borrow_el(&self) -> Ref<T> { RefCell::borrow(&self.0) }
	pub fn borrow_mut_el(&self) -> RefMut<T> { RefCell::borrow_mut(&self.0) }

	fn as_element(&self) -> Ref<dyn ElementTrait> where T: Sized + ElementTrait {
		self.borrow_el() // TODO: unnecessary in here
	}
}

impl<T> Eq for El<T> {}
impl<T> PartialEq for El<T> {
	fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}

impl<T: std::fmt::Debug> std::fmt::Debug for El<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.borrow_el().fmt(f)
	}
}