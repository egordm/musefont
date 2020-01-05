use crate::score::*;
use crate::font::SymName;
use crate::remove_element;

#[derive(Debug, Clone)]
pub struct Rest {
	element: ElementData,
	duration_data: DurationElementData,
	rest_data: ChordRestData,

	sym: SymName,
	/// depends on rest symbol
	dotline: i32,
	/// width of multi measure rest
	mm_width: f32,
	/// invisible and not selectable for user
	gap: bool,
	dots: Vec<El<NoteDot>>
}

impl Element for Rest {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Rest }
}

impl DurationElement for Rest {
	fn duration_data(&self) -> &DurationElementData { &self.duration_data }
	fn duration_data_mut(&mut self) -> &mut DurationElementData { &mut self.duration_data }
}

impl ChordRestTrait for Rest {
	fn rest_data(&self) -> &ChordRestData { &self.rest_data }
	fn rest_data_mut(&mut self) -> &mut ChordRestData { &mut self.rest_data }
}

impl SegmentTrait for Rest {
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CrossMeasure {
	Unknown = -1,
	None = 0,
	First = 1,
	Second = 2
}

#[derive(Debug, Clone)]
pub struct ChordRestData {
	pub(super) elements: Vec<ElementRef>,
	pub(super) duration_type: Duration,
	/// -1, 0, +1, used for crossbeaming
	pub(super) staff_move: i32,

	// TODO: tab duraiton symbol && lyrics

	pub(super) beam: Option<ElWeak<Beam>>,
	pub(super) beam_mode: BeamMode,

	/// actual stem direction
	pub(super) up: bool,
	pub(super) small: bool,

	/// CrossMeasure: combine 2 tied notes if across a bar line and can be combined in a single duration
	/// 0: no cross-measure modification; 1: 1st note of a mod.; -1: 2nd note
	pub(super) cross_measure: CrossMeasure,
	/// the total Duration type of the combined notes
	pub(super) cross_measure_tdur: Duration,
}

impl Default for ChordRestData {
	fn default() -> Self { Self {
		elements: vec![],
		duration_type: Default::default(),
		staff_move: 0,
		beam: None,
		beam_mode: BeamMode::Auto,
		up: true,
		small: false,
		cross_measure: CrossMeasure::Unknown,
		cross_measure_tdur: Default::default()
	}}
}

pub trait ChordRestTrait: DurationElement + SegmentTrait {
	fn rest_data(&self) -> &ChordRestData;
	fn rest_data_mut(&mut self) -> &mut ChordRestData;

	fn elements(&self) -> &Vec<ElementRef> { &self.rest_data().elements }
	//fn set_elements(&mut self, v: Vec<ElementRef>) { self.rest_data_mut().elements = v }
	fn add_element(&mut self, e: ElementRef) { self.rest_data_mut().elements.push(e) }
	fn remove_element(&mut self, e: &ElementRef) { remove_element(&mut self.rest_data_mut().elements, &e) }

	fn duration_type(&self) -> &Duration { &self.rest_data().duration_type }
	fn set_duration_type(&mut self, v: Duration) { self.rest_data_mut().duration_type = v }
	fn set_dots(&mut self, n: u8) { self.rest_data_mut().duration_type.set_dots(n)}
	fn dots(&self) -> u8 {
		if self.cross_measure() == CrossMeasure::First { self.cross_measure_tdur().dots() }
		else if self.cross_measure() == CrossMeasure::Second { 0 }
		else { self.duration_type().dots() }
	}

	fn staff_move(&self) -> i32 { self.rest_data().staff_move }
	fn set_staff_move(&mut self, v: i32) { self.rest_data_mut().staff_move = v }
	
	fn beam(&self) -> Option<El<Beam>> { self.rest_data().beam.as_ref().and_then(|e| e.upgrade()) }
	fn set_beam(&mut self, v: Option<El<Beam>>) { self.rest_data_mut().beam = v.map(|e| e.downgrade()) }
	fn beam_mode(&self) -> BeamMode { self.rest_data().beam_mode }
	fn set_beam_mode(&mut self, v: BeamMode) { self.rest_data_mut().beam_mode = v }
	
	fn up(&self) -> bool { self.rest_data().up }
	fn set_up(&mut self, v: bool) { self.rest_data_mut().up = v }
	fn small(&self) -> bool { self.rest_data().small }
	fn set_small(&mut self, v: bool) { self.rest_data_mut().small = v }
	
	fn cross_measure(&self) -> CrossMeasure { self.rest_data().cross_measure }
	fn set_cross_measure(&mut self, v: CrossMeasure) { self.rest_data_mut().cross_measure = v }
	fn cross_measure_tdur(&self) -> &Duration { &self.rest_data().cross_measure_tdur }
	fn set_cross_measure_tdur(&mut self, v: Duration) { self.rest_data_mut().cross_measure_tdur = v }

	fn get_chordrest_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Small => self.small().into(),
			PropertyId::BeamMode => ValueVariant::from_enum(self.beam_mode()),
			PropertyId::StaffMove => self.staff_move().into(),
			//PropertyId::DurationType => ValueVariant::from_enum(self.duration_type()), TODO
			_ => ValueVariant::None
		}
	}
	fn set_chordrest_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Small => v.with_value(|v| self.set_small(v)),
			PropertyId::BeamMode => v.with_enum(|v| self.set_beam_mode(v)),
			PropertyId::StaffMove => v.with_value(|v| self.set_staff_move(v)),
			//PropertyId::DurationType => v.with_enum(|v| self.set_duration_type(v)),
			_ => false,
		}
	}
}