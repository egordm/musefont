use std::convert::{TryFrom, TryInto};
use crate::score::*;

#[derive(Clone, Debug)]
pub struct ScoreElementData {
	score: Score,
	parent: Option<ElementRefWeak>,
	self_ref: Option<ElementRefWeak>,
}

impl ScoreElementData {
	pub(crate) fn new(score: Score) -> Self {Self {
		score,
		parent: None,
		self_ref: None
	}}
}

pub trait ScoreElement {
	fn sc_data(&self) -> &ScoreElementData;
	fn sc_data_mut(&mut self) -> &mut ScoreElementData;

	/// Gets score element is attached to
	fn score(&self) -> &Score { &self.sc_data().score }

	/// Gets parent of the current element
	/// Warning: Don't take mutable reference. Doing the will avoid a lot of panics
	fn parent(&self) -> Option<ElementRef> { self.sc_data().parent.as_ref().and_then(ElementRefWeak::upgrade) }
	fn set_parent(&mut self, e: Option<ElementRefWeak>) { self.sc_data_mut().parent = e; }
	fn parent_ty<T>(&self) -> Option<El<T>> where Self: Sized, ElementRef: TryInto<El<T>> {
		self.parent().and_then(|e| e.try_into().ok())
	}

	/// Returns a weak reference to self
	fn get_ref(&self) -> ElementRefWeak { self.sc_data().self_ref.clone().expect("Self Ref is not set or invalid!") }
	fn set_ref(&mut self, v: ElementRefWeak) { self.sc_data_mut().self_ref = Some(v) }
}