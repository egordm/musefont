use crate::*;
use crate::score::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct MeasureData {
	/// Measure(/tick) relative -elements: with defined start time but outside the staff
	elements: Vec<ElementRef>,
	pub(crate) time: Fraction,
	/// actual length of measure
	duration: Fraction,
	/// Measure number, counting from zero
	number: i32,
	/// Offset to measure number
	number_offset: i32,
}

impl Default for MeasureData {
	fn default() -> Self { Self {
		elements: vec![],
		time: Fraction::new(0, 1),
		duration: Fraction::new(0, 1),
		number: 0,
		number_offset: 0
	}}
}

pub trait MeasureTrait: Element {
	fn measure_data(&self) -> &MeasureData;
	fn measure_data_mut(&mut self) -> &mut MeasureData;

	fn system(&self) -> Option<El<System>> { self.parent().and_then(|e| e.try_into().ok()) }

	fn elements(&self) -> &Vec<ElementRef> { &self.measure_data().elements }
	fn set_elements(&mut self, v: Vec<ElementRef>) { self.measure_data_mut().elements = v }
	fn add_element(&mut self, e: ElementRef) { self.measure_data_mut().elements.push(e) }
	fn remove_element(&mut self, e: &ElementRef) { remove_element(&mut self.measure_data_mut().elements, e) }

	// TODO: overload time
	fn set_time(&mut self, v: Fraction) { self.measure_data_mut().time = v }
	fn duration(&self) -> Fraction { self.measure_data().duration }
	fn set_duration(&mut self, v: Fraction) { self.measure_data_mut().duration = v }
	fn end_time(&self) -> Fraction { self.time() + self.duration() }

	fn number(&self) -> i32 { self.measure_data().number }
	fn set_number(&mut self, v: i32) { self.measure_data_mut().number = v }
	fn number_offset(&self) -> i32 { self.measure_data().number_offset }
	fn set_number_offset(&mut self, v: i32) { self.measure_data_mut().number_offset = v }

	// Flags
	fn repeat_end(&self) -> bool { self.flag(ElementFlags::REPEAT_END) }
	fn set_repeat_end(&mut self, v: bool) { self.set_flag(ElementFlags::REPEAT_END, v) }
	fn repeat_start(&self) -> bool { self.flag(ElementFlags::REPEAT_START) }
	fn set_repeat_start(&mut self, v: bool) { self.set_flag(ElementFlags::REPEAT_START, v) }
	fn repeat_jump(&self) -> bool { self.flag(ElementFlags::REPEAT_JUMP) }
	fn set_repeat_jump(&mut self, v: bool) { self.set_flag(ElementFlags::REPEAT_JUMP, v) }
	fn irregular(&self) -> bool { self.flag(ElementFlags::IRREGULAR) }
	fn set_irregular(&mut self, v: bool) { self.set_flag(ElementFlags::IRREGULAR, v) }
	fn line_break(&self) -> bool { self.flag(ElementFlags::LINE_BREAK) }
	fn set_line_break(&mut self, v: bool) { self.set_flag(ElementFlags::LINE_BREAK, v) }
	fn page_break(&self) -> bool { self.flag(ElementFlags::PAGE_BREAK) }
	fn set_page_break(&mut self, v: bool) { self.set_flag(ElementFlags::PAGE_BREAK, v) }
	fn section_break(&self) -> bool { self.flag(ElementFlags::SECTION_BREAK) }
	fn set_section_break(&mut self, v: bool) { self.set_flag(ElementFlags::SECTION_BREAK, v) }
	fn no_break(&self) -> bool { self.flag(ElementFlags::NO_BREAK) }
	fn set_no_break(&mut self, v: bool) { self.set_flag(ElementFlags::NO_BREAK, v) }
	fn has_courtesy_key_sig(&self) -> bool { self.flag(ElementFlags::KEYSIG) }
	fn set_has_courtesy_key_sig(&mut self, v: bool) { self.set_flag(ElementFlags::KEYSIG, v) }

	fn index(&self) -> i32 {
		unimplemented!() // TODO: obtain index from the score
	}
	fn measure_index(&self) -> i32 {
		unimplemented!() // TODO: obtain index from the score
	}

	fn get_measure_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::RepeatEnd => self.repeat_end().into(),
			PropertyId::RepeatStart => self.repeat_start().into(),
			PropertyId::RepeatJump => self.repeat_jump().into(),
			PropertyId::NoOffset => self.number_offset().into(),
			PropertyId::Irregular => self.irregular().into(),
			_ => ValueVariant::None
		}
	}
	fn set_measure_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::RepeatEnd => v.with_value(|v| self.set_repeat_end(v)),
			PropertyId::RepeatStart => v.with_value(|v| self.set_repeat_start(v)),
			PropertyId::RepeatJump => v.with_value(|v| self.set_repeat_jump(v)),
			PropertyId::NoOffset => v.with_value(|v| self.set_number_offset(v)),
			PropertyId::Irregular => v.with_value(|v| self.set_irregular(v)),
			_ => false,
		}
	}

	fn add(e: El<Self>, c: ElementRef) where Self: Sized {}

	fn remove(&mut self, e: &ElementRef) { self.base_remove(e) }

	fn base_remove(&mut self, _e: &ElementRef) {
		// TODO: implement layout break remove
	}

	// TODO: next, prev measure
}