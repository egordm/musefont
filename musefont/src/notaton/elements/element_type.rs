use std::{rc::Rc, cell::RefCell};
use crate::*;
use downcast_rs::Downcast;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementType {
	Invalid,
	Chord,
	Note,
	Stem,
	Clef,
	Rest,
	Tie,
	Beam,
	NoteHead,
	NoteDot,
	Symbol,
	Group,
	Accidental,
}

/// Strong reference to an element
pub struct ElementRef<T: ElementTraitDyn + ?Sized>(Rc<RefCell<T>>);

impl<T: ElementTraitDyn + ?Sized> Clone for ElementRef<T> {
	fn clone(&self) -> Self { Self(self.0.clone())}
}

impl<T: ElementTraitDyn + ?Sized> std::fmt::Debug for ElementRef<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let el = RefCell::borrow(&self.0);
		f.debug_struct("Element")
			.field("type", &el.element_type_dyn())
			.finish()
	}
}

impl<T: ElementTraitDyn + ?Sized> ElementRef<T> {
	pub fn val(&self) -> &T { unsafe { &*RefCell::as_ptr(&self.0) } }
	pub fn val_mut(&mut self) -> &mut T { unsafe { &mut *RefCell::as_ptr(&self.0) } }
}

impl<T: ElementTraitDyn + ?Sized> ElementTrait for ElementRef<T> {
	fn el(&self) -> &Element { self.val().el() }
	fn el_mut(&mut self) -> &mut Element { self.val_mut().el_mut() }
	fn element_type() -> ElementType where Self: Sized { ElementType::Invalid }
}

// Strong reference to a generic element
pub type ElementTRef = ElementRef<dyn ElementTraitDyn>;

impl ElementRef<dyn ElementTraitDyn> {
	pub fn downcast_ref<T: 'static + ElementTraitDyn>(&self) -> Option<&T> { self.0.as_any().downcast_ref() }
	pub fn downcast_mut<T: 'static + ElementTraitDyn>(&mut self) -> Option<&mut T> { self.0.as_any_mut().downcast_mut() }
}