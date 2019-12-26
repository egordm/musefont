use crate::*;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct ScoreElement {
	score: Option<Score>,
	id: Option<ElemId>,
	parent: Option<ElemId>
}

impl Default for ScoreElement {
	fn default() -> Self { Self { score: None, id: None, parent: None } }
}

impl ScoreElementTrait for ScoreElement {
	fn sc_el(&self) -> &ScoreElement { self }
	fn sc_el_mut(&mut self) -> &mut ScoreElement { self }
}

pub trait ScoreElementTrait {
	fn sc_el(&self) -> &ScoreElement;
	fn sc_el_mut(&mut self) -> &mut ScoreElement;

	fn attach(&mut self, data: Option<(Score, ElemId)>) {
		let score = self.sc_el_mut();
		if let Some((sc, id)) = data {
			score.score = Some(sc);
			score.id = Some(id);
		} else {
			score.score = None;
			score.id = None;
		}
	}
	fn id(&self) -> Option<ElemId> { self.sc_el().id }
	fn score(&self) -> Option<&Score> { self.sc_el().score.as_ref() }

	fn parent(&self) -> Option<&ElementRef> {
		self.sc_el().parent.and_then(|pid| {
			self.score().and_then(|s| s.get_element(pid))
		})
	}
	fn set_parent<T: ScoreElementTrait>(&mut self, e: Option<&T>) {
		self.sc_el_mut().parent = e.and_then(ScoreElementTrait::id)
	}

	fn neighbor(&self, id: ElemId) -> Option<&ElementRef> { self.score().and_then(|e| e.get_element(id)) }
	fn expect_neighbor<T: ElementTrait>(&self, id: ElemId) -> &T {
		T::from_ref(self.neighbor(id).expect("Element doesn't exist in the score."))
			.expect("Element doesn't have the expected type")
	}
}


/// Iterator transforming element id's into typed elements.
/// Warning: All elements MUST be of this type.
pub struct TypedElementIter<'a, T, I>
	where T: ElementTrait + 'a, I: Iterator<Item=ElemId>
{
	it: I,
	score: &'a Score,
	_phantoms: PhantomData<T>,
}

impl<'a, T, I> TypedElementIter<'a, T, I>
	where T: ElementTrait + 'a, I: Iterator<Item=ElemId>
{
	pub fn new(it: I, score: &'a Score) -> Self { Self { it, score, _phantoms: PhantomData }}
}

impl<'a, T, I> ExactSizeIterator for TypedElementIter<'a, T, I>
	where T: ElementTrait + 'a, I: Iterator<Item=ElemId> + ExactSizeIterator {}

impl<'a, T, I> Iterator for TypedElementIter<'a, T, I> where T: ElementTrait + 'a, I: Iterator<Item=ElemId> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.it.next().and_then(|id| {
			self.score.get_element(id)
				.map(|e| T::from_ref(e).expect("Element doenst have the expected type!"))
		})
	}

	fn size_hint(&self) -> (usize, Option<usize>) { self.it.size_hint() }
}

