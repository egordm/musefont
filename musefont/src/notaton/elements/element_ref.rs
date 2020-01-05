use std::{rc::{Rc, Weak}, cell::{RefCell, Ref, RefMut}};
use crate::*;
use std::ops::Deref;
use std::any::Any;

pub trait RefableElement {
	fn from_ref(r: &ElementRef) -> Option<&Self> where Self: Sized { Self::from_ref_rc(r).map(|e| e.as_ref()) }
	fn from_ref_mut(r: &mut ElementRef) -> Option<&mut Self> where Self: Sized { Self::from_ref_rc(r).map(|e| e.as_mut()) }
	fn from_ref_rc(r: &ElementRef) -> Option<&Elem<Self>> where Self: Sized;
	fn into_ref(self) -> Option<ElementRef>;
	fn transform_ref(r: Elem<Self>) -> Option<ElementRef> where Self: Sized;
}

#[derive(Clone)]
pub struct WeakElem<T>(Weak<RefCell<T>>);

impl<T> std::fmt::Debug for WeakElem<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "Weak")
	}
}

impl<T> WeakElem<T> {
	pub fn upgrade(&self) -> Option<Elem<T>> {
		Weak::upgrade(&self.0).map(Elem)
	}
}

impl<T> PartialEq for WeakElem<T> {
	fn eq(&self, other: &Self) -> bool { Weak::ptr_eq(&self.0, &other.0) }
}

#[derive(Clone)]
pub struct Elem<T>(Rc<RefCell<T>>);

impl<T> Elem<T> {
	pub fn downgrade(&self) -> WeakElem<T> {
		WeakElem(Rc::downgrade(&self.0))
	}

	fn test(&self) -> impl Deref<Target=T> + '_ { RefCell::borrow(&self.0) }

	fn aab(&self) -> Ref< dyn ElementTrait> where T: Sized + ElementTrait {
		Ref::map(RefCell::borrow(&self.0), |v| v)
	}

	fn aa(&self) -> Ref<'_, RectF> where T: Sized + ElementTrait {
		Ref::map(RefCell::borrow(&self.0), |v| v.bbox())
	}
}

impl<T> PartialEq for Elem<T> {
	fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}

impl<T: ScoreElementTrait + RefableElement + Clone> Elem<T> {
	pub fn new(e: T) -> Self {
		let mut ret = Self(Rc::new(RefCell::new(e)));
		let aa = ret.clone();
		let self_ref = aa.into_ref().expect("Element should return a valid ref").downgrade();
		ret.borrow_mut().set_self_ref(self_ref);
		ret
	}
}

impl<T> Elem<T> {
	pub fn borrow(&self) -> &T { self.as_ref() }
	pub fn borrow_mut(&self) -> &mut T { self.as_mut() }
//	pub fn borrow(&self) -> Ref<T> { RefCell::borrow(&self.0) }
//	pub fn borrow_mut(&self) -> RefMut<T> { RefCell::borrow_mut(&self.0) }

	pub(crate) fn as_ref(&self) -> &T { unsafe { &*RefCell::as_ptr(&self.0) } }
	pub(crate) fn as_mut(&self) -> &mut T { unsafe { &mut *RefCell::as_ptr(&self.0) } }
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
	fn into_ref(self) -> Option<ElementRef> { T::transform_ref(self) }
	fn transform_ref(_r: Elem<Self>) -> Option<ElementRef> { None }
}

impl<T: Drawable> Drawable for Elem<T> {
	fn layout(&mut self) {
		self.borrow_mut().layout()
	}
	fn draw(&self, painter: PainterRef) {
		self.borrow_mut().draw(painter)
	}
}

macro_rules! decl_elem_ref {
	{$($Variant:ident($Type:ty)),* $(,)*} => {
		#[derive(Clone, Debug)]
		pub enum ElementWeakRef {
			$(
				$Variant(WeakElem<$Type>)
			),*
		}

		impl ElementWeakRef {
			pub fn upgrade(&self) -> Option<ElementRef> {
				match self {$(
				    ElementWeakRef::$Variant(wc) => { wc.upgrade().map(ElementRef::$Variant) },
				)*}
			}
		}

		#[derive(Clone, PartialEq)]
		pub enum ElementRef {
			$(
				$Variant(Elem<$Type>)
			),*
		}

		impl ElementRef {
			pub fn downgrade(&self) -> ElementWeakRef {
				match self {$(
				    ElementRef::$Variant(rc) => { ElementWeakRef::$Variant(rc.downgrade()) },
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
			fn transform_ref(r: Elem<Self>) -> Option<ElementRef> {
				Some(ElementRef::$Variant(r))
			 }
		})*
	}
}

decl_elem_ref! {
	Symbol(Symbol),
	SymbolGroup(SymbolGroup),
	Accidental(Accidental),
	Beam(Beam),
	Chord(Chord),
	Hook(Hook),
	Measure(Measure),
	MeasureBase(MeasureBase),
	Note(Note),
	NoteDot(NoteDot),
	Part(Part),
//	Rest(Rest),
//	Slur(Slur),
	Staff(Staff),
	Stem(Stem),
//	StemSlash(StemSlash),
//	Tie(Tie),
	TimeSig(TimeSig),
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
	fn transform_ref(_r: Elem<Self>) -> Option<ElementRef> { None }
}