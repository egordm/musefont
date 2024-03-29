use crate::score::*;
use std::convert::{TryFrom, TryInto};

pub trait AtomTrait: Element {
	fn chord(&self) -> Option<El<Chord>> {
		self.parent().and_then(|e|e.try_into().ok())
	}
	fn chord_rest(&self) -> Option<ChordRef> {
		self.parent().and_then(|e| ChordRef::try_from(e).ok())
	}
	fn segment(&self) -> Option<El<Segment>> {
		self.chord_rest()?.as_trait().segment()
	}
	fn measure(&self) -> Option<MeasureRef> {
		MeasureRef::try_from(self.segment()?.borrow_el().parent()?).ok()
	}
	fn system(&self) -> Option<El<System>> {
		self.measure()?.as_trait().parent()?.try_into().ok()
	}
}
