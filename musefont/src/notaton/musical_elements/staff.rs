use crate::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Staff {
	element: Element,

	clefs: ClefList,
	default_clef_type: ClefTypeList,

	keys: KeyList,
	timesigs: BTreeMap<i32, Elem<TimeSig>>,

	/// true - span barline to next staff
	bar_line_span: i32,
	/// line of start staff to draw the barline from (0 = staff top line, ...)
	bar_line_from: i32,
	/// line of end staff to draw the bar line to (0= staff bottom line, ...)
	bar_line_to: i32,

	color: Color,

	staff_type_list: StaffTypeList
}

impl Staff {
	pub fn new(score: Score) -> Elem<Self> { Elem::new(Self {
		element: Element::new(score),
		clefs: ClefList::new(),
		default_clef_type: ClefTypeList::default(),
		keys: KeyList::new(),
		timesigs: Default::default(),
		bar_line_span: 0,
		bar_line_from: 0,
		bar_line_to: 0,
		color: [0, 0, 0, 0xFF],
		staff_type_list: StaffTypeList::new()
	})}

	pub fn part(&self) -> Option<Elem<Part>> { self.parent_ty() }
}

impl ElementTrait for Staff {
	fn el(&self) -> &Element { &self.element }
	fn el_mut(&mut self) -> &mut Element { &mut self.element }
	fn element_type(&self) -> ElementType { ElementType::Staff }
}

#[derive(Clone, Debug)]
pub struct StaffTypeList(BTreeMap<i32, StaffType>);

impl StaffTypeList {
	pub fn new() -> Self { Self(BTreeMap::new()) }

	pub fn staff_type(&self, f: &Fraction) -> &StaffType {
		let tick = f.ticks();
		self.0.range(tick + 1..).next_back().unwrap().1
	}
	pub fn set_staff_type(&mut self, f: &Fraction, v: StaffType) {
		let tick = f.ticks();
		if let Some(current) = self.0.get_mut(&tick) { *current = v; }
		else { self.0.insert(tick, v); }
	}
}