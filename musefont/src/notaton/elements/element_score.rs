use crate::*;

#[derive(Clone, Debug)]
pub struct ScoreElement {
	score: Option<Score>,
}

impl Default for ScoreElement {
	fn default() -> Self { Self { score: None }}
}


impl ScoreElementTrait for ScoreElement {
	fn sc_el(&self) -> &ScoreElement { self }
	fn sc_el_mut(&mut self) -> &mut ScoreElement { self }
}

pub trait ScoreElementTrait {
	fn sc_el(&self) -> &ScoreElement;
	fn sc_el_mut(&mut self) -> &mut ScoreElement;

	fn score(&self) -> Option<&Score> { self.sc_el().score.as_ref() }
}