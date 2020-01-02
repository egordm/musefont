use super::*;
use super::super::*;
use std::convert::TryInto;

macro_rules! decl_elem_ref {{
	enum ($RefName: ident, $RefNameWeak: ident, $RefTypeName: ident) {
		$($Variant:ident($Type:ty)),* $(,)*
	}
} => {
	#[derive(Clone, Debug, Copy, Eq, PartialEq)]
	pub enum $RefTypeName {
		Invalid,
		$($Variant),*
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	pub enum $RefName {
		$($Variant(El<$Type>)),*
	}
	impl $RefName {
		pub fn downgrade(&self) -> $RefNameWeak {
			match self {$(
			    Self::$Variant(r) => $RefNameWeak::$Variant(r.downgrade()),
			)*}
		}
		pub fn get_type(&self) -> $RefTypeName {
			match self {$(
			    Self::$Variant(_) => $RefTypeName::$Variant,
			)*}
		}
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	pub enum $RefNameWeak {
		$($Variant(ElWeak<$Type>)),*
	}
	impl $RefNameWeak {
		pub fn upgrade(&self) -> Option<$RefName> {
			match self {$(
			    Self::$Variant(r) => r.upgrade().map($RefName::$Variant),
			)*}
		}
		pub fn get_type(&self) -> $RefTypeName {
			match self {$(
			    Self::$Variant(_) => $RefTypeName::$Variant,
			)*}
		}
	}

	// Conversion
	$(impl From<El<$Type>> for $RefName {
		fn from(r: El<$Type>) -> Self { $RefName::$Variant(r) }
	})*
	$(impl<'a> TryInto<&'a El<$Type>> for &'a $RefName {
		type Error = ();
		fn try_into(self) -> Result<&'a El<$Type>, Self::Error> {
			if let $RefName::$Variant(r) = self { Ok(r) } else { Err(()) }
		}
	})*


}}

decl_elem_ref! { enum (ElementRef, ElementRefWeak, ElementType) {
	// Atoms
	Accidental(Accidental),
	Articulation(Articulation),
	Chordline(Chordline),
	Hook(Hook),
	Note(Note),
	NoteDot(NoteDot),
	Notehead(Notehead),
	Stem(Stem),
	StemSlash(StemSlash),
	Symbol(Symbol),
	SymbolGroup(SymbolGroup),

	// Composites
	Measure(Measure),
	Part(Part),
	Staff(Staff),
	VBox(VBox),
	HBox(HBox),
	Tuplet(Tuplet),

	// Segments
	Barline(Barline),
	Chord(Chord),
	Clef(Clef),
	KeySig(KeySig),
	Rest(Rest),
	TimeSig(TimeSig),

	// Spanners
	Beam(Beam),
	Line(Line),
	Slur(Slur),
	Tie(Tie),
}}

decl_elem_ref! { enum (MeasureRef, MeasureRefWeak, MeasureType) {
	Measure(Measure),
	VBox(VBox),
	HBox(HBox),
}}

decl_elem_ref! { enum (SegmentRef, SegmentRefWeak, SegmentType) {
	Barline(Barline),
	Chord(Chord),
	Clef(Clef),
	KeySig(KeySig),
	Rest(Rest),
	TimeSig(TimeSig),
}}

decl_elem_ref! { enum (SpannerRef, SpannerRefWeak, SpannerType) {
	Beam(Beam),
	Line(Line),
	Slur(Slur),
	Tie(Tie),
}}