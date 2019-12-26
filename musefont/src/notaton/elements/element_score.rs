use crate::*;

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

	fn parent(&self) -> Option<ElementRef> {
		self.sc_el().parent.and_then(|pid| {
			self.score().and_then(|s| s.get_element(pid))
		})
	}
	fn set_parent<T: ScoreElementTrait>(&mut self, e: Option<&T>) {
		self.sc_el_mut().parent = e.and_then(ScoreElementTrait::id)
	}
}