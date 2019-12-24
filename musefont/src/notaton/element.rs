use crate::*;
use std::rc::Rc;
use downcast_rs::Downcast;
use std::borrow::Borrow;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementType {
	Invalid,
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

#[derive(Clone)]
pub struct ElementRef(Rc<dyn ElementTrait>);

impl std::fmt::Debug for ElementRef {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		let el: &dyn ElementTrait = Rc::borrow(&self.0);
		f.debug_struct("Element")
			.field("type", &el.element_type())
			.field("data", &el.el())
			.finish()
	}
}

impl ElementRef {
	pub fn downcast_ref<T: 'static + ElementTrait>(&self) -> Option<&T> {
		self.0.as_any().downcast_ref()
	}

	pub fn downcast_mut<T: 'static + ElementTrait>(&mut self) -> Option<&mut T> {
		self.0.as_any_mut().downcast_mut()
	}

	pub fn downcast<T: 'static + ElementTrait>(&self) -> Option<Rc<T>> {
		self.0.clone().into_any_rc().downcast().ok()
	}
}

#[derive(Clone, Debug)]
pub struct Element {
	parent: Option<ElementRef>,
	bbox: RectF,
	scale: Size2F,
	pos: Point2F,
	offset: Point2F,
	min_dist: f32,
}

impl Default for Element {
	fn default() -> Self {
		Self {
			parent: None,
			bbox: RectF::default(),
			scale: SIZE_ONE,
			pos: Point2F::default(),
			offset: Point2F::default(),
			min_dist: 0.
		}
	}
}

impl ElementTrait for Element {
	fn el(&self) -> &Element { self }

	fn el_mut(&mut self) -> &mut Element { self }

	fn element_type(&self) -> ElementType { ElementType::Invalid }
}

pub trait ElementTrait: Downcast {
	fn el(&self) -> &Element;
	fn el_mut(&mut self) -> &mut Element;

	fn element_type(&self) -> ElementType;
	fn parent(&self) -> &Option<ElementRef> { &self.el().parent }

	fn ipos(&self) -> &Point2F { &self.el().pos }
	fn pos(&self) -> Point2F { self.el().pos + self.el().offset.to_vector() }
	fn x(&self) -> f32 { self.el().pos.x + self.el().offset.x }
	fn y(&self) -> f32 { self.el().pos.y + self.el().offset.y }
	fn set_pos(&mut self, pos: &Point2F) { self.el_mut().pos = *pos; }
	fn move_pos(&mut self, dt: &Point2F) { self.el_mut().pos += dt.to_vector(); }

	fn scale(&self) -> &Size2F { &self.el().scale }
	fn set_scale(&mut self, scale: &Size2F) { self.el_mut().scale = *scale; }

	fn offset(&self) -> &Point2F { &self.el().offset }
	fn set_offset(&mut self, v: &Point2F) { self.el_mut().offset = *v; }

	fn bbox(&self) -> &RectF { &self.el().bbox }
	fn set_bbox(&mut self, v: &RectF) { self.el_mut().bbox = *v; }
	fn add_bbox(&mut self, v: &RectF) { self.el_mut().bbox = self.el_mut().bbox.union(v); }
	fn width(&self) -> f32 { self.el().bbox.size.width }
	fn set_width(&mut self, v: f32) { self.el_mut().bbox.size.width = v; }
	fn height(&self) -> f32 { self.el().bbox.size.height }
	fn set_height(&mut self, v: f32) { self.el_mut().bbox.size.height = v; }
	fn contains(&self, p: &Point2F) -> bool { self.el().bbox.contains(*p) }
	fn intersects(&self, r: &RectF) -> bool { self.el().bbox.intersects(r) }

	fn baseline(&self) -> f32 { -self.height() }

	// TOOD: part, voice, staff, bar
}

#[derive(Clone, Debug)]
pub struct ElementGroup {
	element: Element,
	leafs: Vec<ElementRef>,
}

impl ElementTrait for ElementGroup {
	fn el(&self) -> &Element { &self.element }

	fn el_mut(&mut self) -> &mut Element { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Group }
}