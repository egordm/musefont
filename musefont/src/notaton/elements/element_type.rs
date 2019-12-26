use std::{rc::Rc, cell::RefCell};
use crate::*;

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

macro_rules! decl_elem_ref {
	{$($Variant:ident($Type:ty)),* $(,)*} => {
		#[derive(Clone)]
		pub enum ElementRef {
			$(
				$Variant(Rc<RefCell<$Type>>)
			),*
		}

		impl ElementRef {
			fn to_el(&self) -> &Element {
				match self {$(
				    ElementRef::$Variant(rc) => { Self::to_ref(rc).el() },
				)*}
			}

			fn to_el_mut(&self) -> &mut Element {
				match self {$(
				    ElementRef::$Variant(rc) => { Self::to_mut(rc).el_mut() },
				)*}
			}
		}

		$(impl RefableElement for $Type {
			fn from_ref(r: &ElementRef) -> Option<&Self> {
				if let ElementRef::$Variant(s) = r { Some(ElementRef::to_ref(s)) }
				else { None }
			}
			fn from_ref_mut(r: &mut ElementRef) -> Option<&mut Self> {
				if let ElementRef::$Variant(s) = r { Some(ElementRef::to_mut(s)) }
				else { None }
			}
			fn into_ref(self) -> Option<ElementRef> {
				Some(ElementRef::$Variant(Rc::new(RefCell::new(self))))
			}
		})*
	}
}

decl_elem_ref!{
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

pub trait RefableElement {
	fn from_ref(r: &ElementRef) -> Option<&Self>;
	fn from_ref_mut(r: &mut ElementRef) -> Option<&mut Self>;
	fn into_ref(self) -> Option<ElementRef>;
}

impl ElementRef {
	fn to_ref<T: ElementTrait>(e: &Rc<RefCell<T>>) -> &T { unsafe { &*RefCell::as_ptr(e) } }
	fn to_mut<T: ElementTrait>(e: &Rc<RefCell<T>>) -> &mut T { unsafe { &mut *RefCell::as_ptr(e) } }
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
	fn element_type(&self) -> ElementType where Self: Sized { ElementType::Invalid }
}

impl RefableElement for ElementRef {
	fn from_ref(r: &ElementRef) -> Option<&Self> { Some(r) }
	fn from_ref_mut(r: &mut ElementRef) -> Option<&mut Self> { Some(r) }
	fn into_ref(self) -> Option<ElementRef> { Some(self) }
}