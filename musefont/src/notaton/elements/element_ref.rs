use std::{rc::{Rc, Weak}, cell::{RefCell, Ref, RefMut}};
use crate::*;
use std::ops::Deref;

pub trait RefableElement: Sized {
	fn from_ref(r: &ElementRef) -> Option<&Self> { Self::from_ref_rc(r).map(|e| e.as_ref()) }
	fn from_ref_mut(r: &mut ElementRef) -> Option<&mut Self> { Self::from_ref_rc(r).map(|e| e.as_mut()) }
	fn from_ref_rc(r: &ElementRef) -> Option<&Elem<Self>>;
	fn into_ref(self) -> Option<ElementRef>;
}

#[derive(Clone)]
pub struct Elem<T>(Rc<RefCell<T>>);

impl<T> Elem<T> {
	pub fn new(e: T) -> Self { Self(Rc::new(RefCell::new(e))) }
	pub fn borrow(&self) -> Ref<T> { RefCell::borrow(&self.0) }
	pub fn borrow_mut(&self) -> RefMut<T> { RefCell::borrow_mut(&self.0) }

	fn as_ref(&self) -> &T { unsafe { &*RefCell::as_ptr(&self.0) } }
	fn as_mut(&self) -> &mut T { unsafe { &mut *RefCell::as_ptr(&self.0) } }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Elem<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Debug::fmt(self.borrow().deref(), f)
	}
}

impl<T: ElementTrait> ElementTrait for Elem<T> {
	fn el(&self) -> &Element { self.as_ref().el() }
	fn el_mut(&mut self) -> &mut Element { self.as_mut().el_mut() }
	fn element_type(&self) -> ElementType where Self: Sized { self.as_ref().element_type() }
}

impl<T: RefableElement> RefableElement for Elem<T> {
	fn from_ref_rc(_r: &ElementRef) -> Option<&Elem<Self>> { None }
	fn into_ref(self) -> Option<ElementRef> { None }
}

macro_rules! decl_elem_ref {
	{$($Variant:ident($Type:ty)),* $(,)*} => {
		#[derive(Clone)]
		pub enum ElementWeakRef {
			$(
				$Variant(Weak<RefCell<$Type>>)
			),*
		}

		impl ElementWeakRef {
			pub fn upgrade(&self) -> Option<ElementRef> {
				match self {$(
				    ElementWeakRef::$Variant(wc) => { wc.upgrade().map(|e| ElementRef::$Variant(Elem(e))) },
				)*}
			}
		}

		#[derive(Clone)]
		pub enum ElementRef {
			$(
				$Variant(Elem<$Type>)
			),*
		}

		impl ElementRef {
			pub fn downgrade(&self) -> ElementWeakRef {
				match self {$(
				    ElementRef::$Variant(rc) => { ElementWeakRef::$Variant(Rc::downgrade(&rc.0)) },
				)*}
			}

			fn to_el(&self) -> &Element {
				match self {$(
				    ElementRef::$Variant(rc) => { rc.as_ref().el() },
				)*}
			}

			fn to_el_mut(&self) -> &mut Element {
				match self {$(
				    ElementRef::$Variant(rc) => { rc.as_mut().el_mut() },
				)*}
			}

			fn get_element_type(&self) -> ElementType {
				match self {$(
				    ElementRef::$Variant(rc) => { rc.as_ref().element_type() },
				)*}
			}
		}

		$(impl RefableElement for $Type {
			fn from_ref_rc(r: &ElementRef) -> Option<&Elem<Self>> {
				if let ElementRef::$Variant(s) = r { Some(s) }
				else { None }
			}
			fn into_ref(self) -> Option<ElementRef> {
				Some(ElementRef::$Variant(Elem::new(self)))
			}
		})*
	}
}

decl_elem_ref! {
	Symbol(Symbol),
	SymbolGroup(SymbolGroup),
	Accidental(Accidental),
//	Beam(Beam),
	Chord(Chord),
//	Hook(Hook),
	Note(Note),
	NoteDot(NoteDot),
//	Rest(Rest),
//	Slur(Slur),
//	Stem(Stem),
//	StemSlash(StemSlash),
//	Tie(Tie),
}

impl std::fmt::Debug for ElementWeakRef {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		std::fmt::Debug::fmt(&self.upgrade(), f)
	}
}


impl std::fmt::Debug for ElementRef {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		f.debug_struct("Element")
			.field("type", &self.element_type())
			.finish()
	}
}

impl ElementTrait for ElementRef {
	fn el(&self) -> &Element { self.to_el() }
	fn el_mut(&mut self) -> &mut Element { self.to_el_mut() }
	fn element_type(&self) -> ElementType { self.get_element_type() }
}

impl RefableElement for ElementRef {
	fn from_ref_rc(_r: &ElementRef) -> Option<&Elem<Self>> { None }
	fn into_ref(self) -> Option<ElementRef> { Some(self) }
}