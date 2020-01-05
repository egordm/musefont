use crate::score::*;

/// Represents a Key Signature on a staff
#[derive(Debug, Clone)]
pub struct KeySig {
	element: ElementData,

	/// Show courtesy key signature for this sig if appropriate
	show_courtesy: bool,
	/// Used in layout to override score style (needed for the Continuous panel)
	hide_naturals: bool,
	sig: KeySigEvent,
}

impl KeySig {
	pub fn new(score: Score) -> El<Self> { new_element(Self {
		element: ElementData::new(score),
		show_courtesy: true,
		hide_naturals: false,
		sig: Default::default()
	})}

	pub fn show_courtesy(&self) -> bool { self.show_courtesy }
	pub fn set_show_courtesy(&mut self, v: bool) { self.show_courtesy = v }
	pub fn hide_naturals(&self) -> bool { self.hide_naturals }
	pub fn set_hide_naturals(&mut self, v: bool) { self.hide_naturals = v }

	pub fn sig(&self) -> &KeySigEvent { &self.sig }
	pub fn set_sig(&mut self, v: KeySigEvent) { self.sig = v }

	pub fn key(&self) -> Key { self.sig.key() }
	pub fn set_key(&mut self, v: Key) { self.sig.set_key(v) }
	pub fn mode(&self) -> KeyMode { self.sig.mode() }
	pub fn set_mode(&mut self, v: KeyMode) { self.sig.set_mode(v) }
	pub fn custom(&self) -> bool { self.sig.custom() }
	pub fn is_valid(&self) -> bool { self.sig.key() != Key::Invalid }
	pub fn is_atonal(&self) -> bool { self.sig.mode() == KeyMode::None }

	fn get_custom_property(&self, p: PropertyId) -> ValueVariant {
		match p {
			PropertyId::Key => ValueVariant::from_enum(self.key()),
			PropertyId::KeysigMode => ValueVariant::from_enum(self.mode()),
			PropertyId::ShowCourtesy => self.show_courtesy().into(),
			_ => ValueVariant::None
		}
	}
	fn set_custom_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		match p {
			PropertyId::Key => v.with_enum(|v| self.set_key(v)),
			PropertyId::KeysigMode => v.with_enum(|v| self.set_mode(v)),
			PropertyId::ShowCourtesy => v.with_value(|v| self.set_show_courtesy(v)),
			_ => false,
		}
	}
}

impl Element for KeySig {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::KeySig }

	fn get_property(&self, p: PropertyId) -> ValueVariant {
		self.get_custom_property(p).if_none(|| self.get_element_property(p))
	}
	fn set_property(&mut self, p: PropertyId, v: ValueVariant) -> bool {
		self.set_element_property(p, v.clone()) || self.set_custom_property(p, v)
	}
}

impl SegmentTrait for KeySig {
}