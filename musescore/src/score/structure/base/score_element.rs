use crate::score::*;
use std::convert::{TryInto};
use crate::font::ScoreFont;

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
	fn score_data(&self) -> &ScoreElementData;
	fn score_data_mut(&mut self) -> &mut ScoreElementData;

	/// Gets score element is attached to
	fn score(&self) -> &Score { &self.score_data().score }
	fn font(&self) -> Ref<ScoreFont> { self.score_data().score.font() }
	fn style(&self) -> Ref<Style> { self.score_data().score.style() }

	/// Gets parent of the current element
	/// Warning: Don't take mutable reference. Doing the will avoid a lot of panics
	fn parent(&self) -> Option<ElementRef> { self.score_data().parent.as_ref().and_then(ElementRefWeak::upgrade) }
	fn set_parent(&mut self, e: Option<ElementRefWeak>) { self.score_data_mut().parent = e; }
	fn parent_ty<T>(&self) -> Option<El<T>> where Self: Sized, ElementRef: TryInto<El<T>> {
		self.parent().and_then(|e| e.try_into().ok())
	}
	fn parent_iter(&self) -> ParentIter { ParentIter(self.score_data().parent.clone()) }

	/// Returns a weak reference to self
	fn get_ref(&self) -> ElementRefWeak { self.score_data().self_ref.clone().expect("Self Ref is not set or invalid!") }
	fn set_ref(&mut self, v: ElementRefWeak) { self.score_data_mut().self_ref = Some(v) }
	fn get_ref_ty<T>(&self) -> Option<El<T>> where Self: Sized, ElementRef: TryInto<El<T>> {
		self.get_ref().upgrade().and_then(|e| e.try_into().ok())
	}
}

pub struct ParentIter(Option<ElementRefWeak>);

impl Iterator for ParentIter {
	type Item = ElementRef;

	fn next(&mut self) -> Option<Self::Item> {
		let ret = self.0.clone().and_then(|e| e.upgrade());
		self.0 = ret.clone().and_then(|e| e.as_trait().score_data().parent.clone());
		ret
	}
}