use crate::*;

#[derive(Clone, Debug)]
pub enum WeakMeasureRef {
	Measure(WeakElem<Measure>)
}

impl WeakMeasureRef {
	pub fn upgrade(&self) -> Option<MeasureRef> {
		match self {
			WeakMeasureRef::Measure(e) => e.upgrade().map(MeasureRef::Measure),
		}
	}
}

#[derive(Clone, Debug)]
pub enum MeasureRef {
	Measure(Elem<Measure>)
}

impl MeasureRef {
	fn to_measure(&self) -> &MeasureBase {
		match self {
			MeasureRef::Measure(rc) => rc.as_ref().measure(),
		}
	}

	fn to_measure_mut(&self) -> &mut MeasureBase {
		match self {
			MeasureRef::Measure(rc) => rc.as_mut().measure_mut(),
		}
	}

	pub fn downgrade(self) -> WeakMeasureRef {
		match self {
			MeasureRef::Measure(e) => WeakMeasureRef::Measure(e.downgrade()),
		}
	}
}

impl MeasureTrait for MeasureRef {
	fn measure(&self) -> &MeasureBase { self.to_measure() }
	fn measure_mut(&mut self) -> &mut MeasureBase { self.to_measure_mut() }
}

/// base for Measure, HBox and VBox
#[derive(Clone, Debug)]
pub struct MeasureBase {
	element: Element,
	next: Option<WeakMeasureRef>,
	prev: Option<WeakMeasureRef>,

	/// Measure(/tick) relative -base: with defined start time
	/// but outside the staff
	children: Vec<ElementRef>,

	tick: Fraction,
	// actual length of measure
	len: Fraction,
	/// Measure number, counting from zero
	no: i32,
	/// Offset to measure number
	no_offset: i32,
}

impl MeasureTrait for MeasureBase {
	fn measure(&self) -> &MeasureBase { self }
	fn measure_mut(&mut self) -> &mut MeasureBase { self }
}

impl ElementTrait for MeasureBase {
	fn el(&self) -> &Element { &self.element }
	fn el_mut(&mut self) -> &mut Element { &mut self.element }
	fn element_type(&self) -> ElementType { ElementType::Invalid }
}

pub trait MeasureTrait {
	fn measure(&self) -> &MeasureBase;
	fn measure_mut(&mut self) -> &mut MeasureBase;

	fn next(&self) -> Option<MeasureRef> { self.measure().next.as_ref().and_then(WeakMeasureRef::upgrade) }
	fn set_next(&mut self, v: Option<MeasureRef>) { self.measure_mut().next = v.map(MeasureRef::downgrade) }
	fn prev(&self) -> Option<MeasureRef> { self.measure().prev.as_ref().and_then(WeakMeasureRef::upgrade) }
	fn set_prev(&mut self, v: Option<MeasureRef>) { self.measure_mut().prev = v.map(MeasureRef::downgrade) }

	fn next_measure(&self) -> Option<Elem<Measure>> {
		let mut m = self.next();
		while let Some(s) = m {
			if let MeasureRef::Measure(r) = s {
				return Some(r);
			}
			m = s.next();
		}
		return None;
	}
	fn prev_measure(&self) -> Option<Elem<Measure>> {
		let mut m = self.prev();
		while let Some(s) = m {
			if let MeasureRef::Measure(r) = s {
				return Some(r);
			}
			m = s.prev();
		}
		return None;
	}

	fn elements(&self) -> &[ElementRef] { &self.measure().children }

	fn add(&mut self, mut el: ElementRef) {
		el.set_parent(Some(self.measure().get_self_ref()));
		// TODO: invalidate layout
		self.measure_mut().children.push(el);
	}
	fn remove(&mut self, el: &ElementRef) {
		if let Some(pos) = self.measure().children.iter().position(|e | e == el) {
			self.measure_mut().children.remove(pos);
			// TODO: invalidate layout
		}
	}
}