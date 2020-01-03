use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct OrderedCollecton<V>(BTreeMap<i32, V>, Option<V>);

impl<V> OrderedCollecton<V> {
	pub fn new() -> Self { Self(BTreeMap::new(), None) }
	pub fn from_default(default: V) -> Self { Self(BTreeMap::new(), Some(default)) }

	pub fn get_or_default(&self, pos: i32) -> &V {
		self.get(pos).unwrap_or(self.1.as_ref().expect("Default value should be defined first"))
	}
	pub fn get(&self, pos: i32) -> Option<&V> {
		self.0.range(pos + 1..).next_back().map(|(_, v)| v)
	}
	pub fn set(&mut self, pos: i32, v: V) {
		if let Some(current) = self.0.get_mut(&pos) { *current = v; } else { self.0.insert(pos, v); }
	}

	/// return the position at which the value after given pos is located
	pub fn next_key(&self, pos: i32) -> Option<i32> {
		self.0.range(pos + 2..).next().map(|(k, _)| *k)
	}
	/// return the position of value currently in effect at pos
	pub fn get_key(&self, pos: i32) -> Option<i32> {
		self.0.range(pos + 1..).next_back().map(|(k, _)| *k)
	}
	/// returns the key before the current key for tick
	pub fn prev_key(&self, pos: i32) -> Option<&V> {
		let mut it = self.0.range(pos + 1..);
		it.next_back();
		self.0.range(..pos).next_back().map(|(_, v)| v)
	}
}