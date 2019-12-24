use crate::*;
use std::rc::Rc;

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
	Accidental,
}

#[derive(Clone)]
pub struct Element {
	parent: Option<Rc<dyn ElementTrait>>,
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

pub trait ElementTrait {
	fn el(&self) -> &Element;
	fn el_mut(&mut self) -> &mut Element;

	fn element_type(&self) -> ElementType;

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