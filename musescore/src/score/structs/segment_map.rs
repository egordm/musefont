use crate::score::*;
use std::collections::BTreeMap;
use std::ops::RangeBounds;
use bitflags::_core::ops::Bound;

const TYPE_MASK: usize = ((1 << 16) - 1);
const TICK_MASK: usize = !TYPE_MASK;

type Key = usize;
type Value = El<Segment>;
type Range<'a> = std::collections::btree_map::Range<'a, Key, Value>;

#[derive(Debug, Clone)]
pub struct SegmentMap {
	data: BTreeMap<Key, Value>
}

impl SegmentMap {
	pub fn new() -> Self { Self { data: BTreeMap::new() }}

	pub fn len(&self) -> usize { self.data.len() }

	pub fn get(&self, t: Fraction) -> Option<&Value> { self.iter_time(t).map(|(_, v)| v).next() }
	pub fn get_ty(&self, t: Fraction, ty: impl Into<SegmentTypeMask>) -> Option<&Value> { self.iter_ty(t, ty).map(|(_, v)| v).next() }

	pub fn range(&self, r: impl RangeBounds<Fraction>) -> Range {
		self.data.range((
			convert_bound(r.start_bound(), |k| Self::key_from_tick(k)),
			convert_bound(r.end_bound(), |k| Self::key_from_tick(k) | TICK_MASK),
			))
	}

	pub fn get_next(&self, t: &Fraction) -> Option<&Value> {
		let t = Self::key_from_tick(&t);
		self.data.range(t..).next().map(|(_, v)| v)
	}
	pub fn get_prev(&self, t: &Fraction) -> Option<&Value> {
		let t = ((t.ticks() + 1) as Key) << 16;
		self.data.range(..t).next_back().map(|(_, v)| v)
	}

	pub fn insert(&mut self, e: Value) {
		let (t, ty) = {
			let r = e.borrow_el();
			(r.rel_time(), r.segment_type())
		};
		self.data.insert(Self::key_from(&t, ty), e);
	}

	pub fn remove(&mut self, e: &Value) {
		let (t, ty) = {
			let r = e.borrow_el();
			(r.time(), r.segment_type())
		};
		self.data.remove(&Self::key_from(&t, ty));
	}

	pub fn iter_time(&self, t: Fraction) -> impl Iterator<Item=(&Key, &Value)> {
		let t = Self::key_from_tick(&t);
		self.data.range(t..).take_while(move |(k, _)| (*k & TICK_MASK) == t)
	}
	pub fn iter_ty(&self, t: Fraction, ty: impl Into<SegmentTypeMask>) -> impl Iterator<Item=(&Key, &Value)> {
		let ty = Self::key_from_type(ty.into());
		self.iter_time(t).filter(move |(k, _)| (*k & ty) > 0)
	}
	pub fn iter_vals(&self) -> impl DoubleEndedIterator<Item=&Value> {
		self.data.iter().map(|(_, v)| v)
	}

	pub fn first(&self) -> Option<&Value> {
		self.data.iter().next().map(|(k, v)| v)
	}

	//pub fn get_ty(&self, t: &Fraction, ty: SegmentTypeMask) -> Option<&El<Segment>>
	/// Gets the real key combining fraction and segment type
	pub fn key_from(t: &Fraction, ty: SegmentTypeMask) -> Key {
		let (ticks, ty_id) = (t.ticks().max(0) as Key, ty.bits() as Key);
		(ticks << 16) | ty_id
	}
	pub fn key_to(k: usize) -> (Fraction, SegmentTypeMask) { (Self::key_tick(k), Self::key_type(k)) }

	pub fn key_tick(k: usize) -> Fraction { Fraction::from_ticks((k >> 16) as i32) }
	pub fn key_from_tick(t: &Fraction) -> Key { (t.ticks().max(0) as Key) << 16 }
	pub fn key_type(k: usize) -> SegmentTypeMask { SegmentTypeMask::from_bits((k & TYPE_MASK) as u16).unwrap() }
	pub fn key_from_type(t: SegmentTypeMask) -> Key { t.bits() as Key }
}

fn convert_bound<T>(b: Bound<T>, f: impl Fn(T) -> Key) -> Bound<Key> {
	match b {
		Bound::Included(k) => Bound::Included(f(k)),
		Bound::Excluded(k) => Bound::Excluded(f(k)),
		Bound::Unbounded => Bound::Unbounded,
	}
}