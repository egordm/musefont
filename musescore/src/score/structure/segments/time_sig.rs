use crate::*;
use crate::score::*;
use crate::font::SymName;

/// This class represents a time signature.
#[derive(Debug, Clone)]
pub struct TimeSig {
	element: ElementData,

	/// calculated from actualSig() if !customText
	numerator_string: String,
	denominator_string: String,

	ns: Vec<SymName>,
	ds: Vec<SymName>,

	pz: Point2F,
	pn: Point2F,

	point_large_left_paren: Point2F,
	point_large_right_paren: Point2F,

	sig: Fraction,
	/// localSig / globalSig
	stretch: Fraction,
	groups: Groups,

	scale: Size2F,
	time_sig_type: TimeSigType,
	show_courtesy_sig: bool,
	large_parentheses: bool,
}

impl TimeSig {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		numerator_string: String::new(),
		denominator_string: String::new(),
		ns: vec![],
		ds: vec![],
		pz: Default::default(),
		pn: Default::default(),
		point_large_left_paren: Default::default(),
		point_large_right_paren: Default::default(),
		sig: Fraction::new(1, 1),
		stretch: Fraction::new(1, 1),
		groups: Groups::new(),
		scale: Default::default(),
		time_sig_type: TimeSigType::Normal,
		show_courtesy_sig: true,
		large_parentheses: false
	})}

	pub fn sig(&self) -> &Fraction { &self.sig }
	pub fn set_sig(&mut self, v: Fraction, st: TimeSigType) {
		self.sig = v;
		self.time_sig_type = st;
		self.large_parentheses = false;
		self.numerator_string.clear();
		self.denominator_string.clear();
	}

	pub fn stretch(&self) -> &Fraction { &self.stretch }
	pub fn set_stretch(&mut self, v: Fraction) { self.stretch = v }

	pub fn scale(&self) -> &Size2F { &self.scale }
	pub fn set_scale(&mut self, v: Size2F) { self.scale = v }

	pub fn groups(&self) -> &Groups { &self.groups }
	pub fn set_groups(&mut self, g: Groups) { self.groups = g }

	pub fn numerator(&self) -> i32 { self.sig.numerator }
	pub fn denominator(&self) -> i32 { self.sig.denominator }
	pub fn numerator_string(&self) -> &String { &self.numerator_string }
	pub fn set_numerator_string(&mut self, v: String) { self.numerator_string = v }
	pub fn denominator_string(&self) -> &String { &self.denominator_string }
	pub fn set_denominator_string(&mut self, v: String) { self.denominator_string = v }

	pub fn large_parentheses(&self) -> bool { self.large_parentheses }
	pub fn set_large_parentheses(&mut self, v: bool) { self.large_parentheses = v }

	pub fn show_courtesy_sig(&self) -> bool { self.show_courtesy_sig }
	pub fn set_show_courtesy_sig(&mut self, v: bool) { self.show_courtesy_sig = v }

	pub fn global_sig(&self) -> Fraction { (self.sig * self.stretch).reduced() }
	pub fn set_global_sig(&mut self, f: Fraction) { self.stretch = (self.sig / f).reduced() }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::ShowCourtesy => self.show_courtesy_sig().into(),
			PropertyId::NumeratorString => self.numerator_string().clone().into(),
			PropertyId::DenominatorString => self.denominator_string().clone().into(),
			PropertyId::Timesig => self.sig().ticks().into(),
			PropertyId::TimesigGlobal => self.global_sig().ticks().into(),
			PropertyId::TimesigStretch => self.stretch().ticks().into(),
			PropertyId::TimesigType => ValueVariant::from_enum(self.time_sig_type),
			PropertyId::Scale => self.scale().to_vector().to_point().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::ShowCourtesy => v.with_value(|v| self.set_show_courtesy_sig(v)),
			PropertyId::NumeratorString => v.with_value(|v| self.set_numerator_string(v)),
			PropertyId::DenominatorString => v.with_value(|v| self.set_denominator_string(v)),
			PropertyId::Timesig => v.with_value(|v| self.set_sig(Fraction::from_ticks(v), self.time_sig_type)),
			PropertyId::TimesigGlobal => v.with_value(|v| self.set_global_sig(Fraction::from_ticks(v))),
			PropertyId::TimesigStretch => v.with_value(|v| self.set_stretch(Fraction::from_ticks(v))),
			PropertyId::TimesigType => v.with_enum(|v| self.time_sig_type = v),
			PropertyId::Scale => v.with_value(|v: Point2F| self.set_scale(Size2F::new(v.x, v.y))),
			_ => false,
		}
	}

}

impl Element for TimeSig {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::TimeSig }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p).if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl SegmentTrait for TimeSig {
}

#[derive(Clone, Copy, Primitive, Debug)]
pub enum TimeSigType {
	/// use sz/sn text
	Normal = 0,
	/// common time (4/4)
	FourFour = 1,
	/// cut time (2/2)
	AllaBreve = 2,
}