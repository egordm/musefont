use crate::*;

#[derive(Clone, Debug)]
pub struct ScoreElement {
	score: Score,
	parent: Option<ElementWeakRef>
}

impl ScoreElement {
	pub fn new(score: Score) -> Self { Self {
		score,
		parent: None
	} }
}

impl ScoreElementTrait for ScoreElement {
	fn sc_el(&self) -> &ScoreElement { self }
	fn sc_el_mut(&mut self) -> &mut ScoreElement { self }
}

pub trait ScoreElementTrait {
	fn sc_el(&self) -> &ScoreElement;
	fn sc_el_mut(&mut self) -> &mut ScoreElement;

	fn score(&self) -> &Score { &self.sc_el().score }

	fn parent(&self) -> Option<ElementRef> { self.sc_el().parent.as_ref().and_then(ElementWeakRef::upgrade) }
	fn set_parent(&mut self, e: Option<ElementWeakRef>) { self.sc_el_mut().parent = e; }
}



/*
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
*/

