use crate::*;
use downcast_rs::Downcast;

#[derive(Clone, Debug)]
pub struct Element {
	sc_element: ScoreElement,
	bbox: RectF,
	scale: Size2F,
	pos: Point2F,
	offset: Point2F,
	min_dist: f32,
}

impl Default for Element {
	fn default() -> Self {
		Self {
			sc_element: ScoreElement::default(),
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

	fn element_type() -> ElementType { ElementType::Invalid }
}

pub trait ElementTrait: Downcast {
	fn el(&self) -> &Element;
	fn el_mut(&mut self) -> &mut Element;

	fn element_type() -> ElementType where Self: Sized;

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

pub trait ElementTraitDyn: ElementTrait {
	fn element_type_dyn(&self) -> ElementType;
}

impl<T: ElementTrait> ElementTraitDyn for T {
	fn element_type_dyn(&self) -> ElementType { T::element_type() }
}

impl<T: ElementTrait> ScoreElementTrait for T {
	fn sc_el(&self) -> &ScoreElement { &self.el().sc_element}
	fn sc_el_mut(&mut self) -> &mut ScoreElement { &mut self.el_mut().sc_element }
}

/*
macro_rules! impl_elem {
	($t:ident, $el_ty:expr) => {
		impl ElementTrait for $t {
			fn el(&self) -> &Element { &self.element }
			fn el_mut(&mut self) -> &mut Element { &mut self.element }
			fn element_type(&self) -> ElementType { $el_ty }
		}
	}
}*/
