use std::{rc::{Weak, Rc}, cell::RefCell};

pub use std::cell::{Ref, RefMut};
use std::convert::TryInto;

#[derive(Clone)]
pub struct ElWeak<T>(Weak<RefCell<T>>);

impl<T> ElWeak<T> {
	pub fn upgrade(&self) -> Option<El<T>> {
		Weak::upgrade(&self.0).map(El)
	}
}

impl<T> TryInto<El<T>> for ElWeak<T> {
	type Error = ();
	fn try_into(self) -> Result<El<T>, Self::Error> { self.upgrade().ok_or(()) }
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

impl<T> From<T> for El<T> {
	fn from(v: T) -> Self { El(Rc::new(RefCell::new(v))) }
}

impl<T> El<T> {
	pub fn downgrade(&self) -> ElWeak<T> { ElWeak(Rc::downgrade(&self.0)) }

	pub fn borrow_el(&self) -> Ref<T> { RefCell::borrow(&self.0) }
	pub fn with<F: FnMut(Ref<T>) -> R, R>(&self, mut f: F) -> R {
		f(self.borrow_el())
	}
	pub fn with_d<F: FnMut(Ref<T>) -> Option<R>, R>(&self, f: F, default: R) -> R {
		self.with(f).unwrap_or(default)
	}

	pub fn borrow_mut_el(&self) -> RefMut<T> { RefCell::borrow_mut(&self.0) }
	pub fn with_mut<F: FnMut(RefMut<T>) -> R, R>(&self, mut f: F) -> R {
		f(self.borrow_mut_el())
	}
	pub fn with_mut_i<F: FnMut(RefMut<T>)>(self, mut f: F) -> Self {
		f(self.borrow_mut_el());
		self
	}
}

impl<T> Into<ElWeak<T>> for El<T> {
	fn into(self) -> ElWeak<T> { self.downgrade() }
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

pub trait OptionalEl<T> {
	fn with<F: FnMut(Ref<T>) -> R, R>(&self, f: F) -> Option<R>;

	fn with_d<F: FnMut(Ref<T>) -> R, R>(&self, f: F, default: R) -> R {
		self.with(f).unwrap_or(default)
	}

	fn with_mut<F: FnMut(RefMut<T>) -> R, R>(&self, f: F) -> Option<R>;

	fn downgrade(self) -> Option<ElWeak<T>>;
}

impl<T> OptionalEl<T> for Option<El<T>> {
	fn with<F: FnMut(Ref<T>) -> R, R>(&self, mut f: F) -> Option<R> {
		Some(f(self.as_ref()?.borrow_el()))
	}

	fn with_mut<F: FnMut(RefMut<T>) -> R, R>(&self, mut f: F) -> Option<R> {
		Some(f(self.as_ref()?.borrow_mut_el()))
	}

	fn downgrade(self) -> Option<ElWeak<T>> { self.as_ref().map(El::downgrade) }
}

impl<T> OptionalEl<T> for Option<&El<T>> {
	fn with<F: FnMut(Ref<T>) -> R, R>(&self, mut f: F) -> Option<R> {
		Some(f(self.as_ref()?.borrow_el()))
	}

	fn with_mut<F: FnMut(RefMut<T>) -> R, R>(&self, mut f: F) -> Option<R> {
		Some(f(self.as_ref()?.borrow_mut_el()))
	}

	fn downgrade(self) -> Option<ElWeak<T>> { self.map(El::downgrade) }
}

pub trait OptionalWeakEl<T> {
	fn upgrade(&self) -> Option<El<T>>;
}

impl<T> OptionalWeakEl<T> for Option<ElWeak<T>> {
	fn upgrade(&self) -> Option<El<T>> {
		if let Some(e) = self {
			e.upgrade()
		} else { None }
	}
}

impl<T> OptionalEl<T> for Option<ElWeak<T>> {
	fn with<F: FnMut(Ref<T>) -> R, R>(&self, f: F) -> Option<R> {
		self.upgrade().with(f)
	}

	fn with_mut<F: FnMut(RefMut<T>) -> R, R>(&self, f: F) -> Option<R> {
		self.upgrade().with_mut(f)
	}

	fn downgrade(self) -> Option<ElWeak<T>> { self }
}