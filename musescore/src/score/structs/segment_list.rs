use crate::score::*;
use std::convert::TryInto;

#[derive(Clone, Debug, Default)]
pub struct SegmentNodeData {
	prev: Option<SegmentRefWeak>,
	next: Option<SegmentRef>,
}

pub trait SegmentNode {
	fn data(&self) -> &SegmentNodeData;
	fn data_mut(&mut self) -> &mut SegmentNodeData;

	fn prev_weak(&self) -> Option<SegmentRefWeak> { self.data().prev.clone() }
	fn prev(&self) -> Option<SegmentRef> { self.prev_weak().and_then(|e| e.upgrade())}

	fn next(&self) -> Option<SegmentRef> { self.data().next.clone() }
}

#[derive(Clone, Debug)]
pub struct SegmentList {
	first: Option<SegmentRef>,
	last: Option<SegmentRef>,
	len: usize,
}

impl SegmentList {
	pub fn new() -> Self { Self { first: None, last: None, len: 0 }}

	pub fn clear(&mut self) {
		self.first = None;
		self.last = None;
		self.len = 0;
	}
	pub fn remove(&mut self, v: &SegmentRef) {
		// TOOD: assert contains.
		let mut v = v.as_trait_mut();
		let SegmentNodeData {ref mut prev, ref mut next}  = v.data_mut();

		if let Some(prev) = prev.as_ref().and_then(SegmentRefWeak::upgrade) {
			prev.as_trait_mut().data_mut().next = next.clone();
		}
		if let Some(next) = next {
			next.as_trait_mut().data_mut().prev = prev.clone();
		}

		*prev = None;
		*next = None;
		self.len -= 1;
		debug_assert!(self.len >= 0)
	}
	pub fn insert(&mut self, prev: &SegmentRef, v: &SegmentRef) {
		let mut pt = v.as_trait_mut();
		let mut vt = v.as_trait_mut();

		vt.data_mut().prev = Some(prev.downgrade());
		vt.data_mut().next = pt.data().next.clone();
		pt.data_mut().next = Some(v.clone());
		self.len += 1;
	}

	pub fn push_back(&mut self, v: SegmentRef) {
		self.len += 1;

		{
			let mut m = v.as_trait_mut();
			m.data_mut().prev = self.last.as_ref().map(SegmentRef::downgrade);
			m.data_mut().next = None;
		}

		if let Some(last) = &self.last {
			last.as_trait_mut().data_mut().next = Some(v.clone());
		}

		self.last = Some(v);
		if self.first.is_none() { self.first = self.last.clone(); }
	}
	pub fn push_front(&mut self, v: SegmentRef) {
		self.len += 1;

		{
			let mut m = v.as_trait_mut();
			m.data_mut().prev = None;
			m.data_mut().next = self.first.clone();
		}

		if let Some(first) = &self.first {
			first.as_trait_mut().data_mut().prev = Some(v.downgrade());
		}

		self.first = Some(v);
		if self.last.is_none() { self.last = self.first.clone(); }
	}

	pub fn len(&self) -> usize { self.len }
}

pub struct SegmentIter(Option<SegmentRef>);

impl Iterator for SegmentIter {
	type Item = SegmentRef;

	fn next(&mut self) -> Option<Self::Item> {
		let ret = self.0.clone();
		if let Some(r) = &ret {
			self.0 = r.as_trait().next()
		}
		ret
	}
}