use super::*;
use super::super::*;
use std::convert::TryInto;
use std::any::Any;

macro_rules! decl_elem_ref {{
	enum ($RefName: ident, $RefNameWeak: ident, $RefTypeName: ident) -> $Trait:ident
	{
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

	// Trait retrieval
	impl $RefName {
		pub fn as_trait(&self) -> Ref<dyn $Trait> {
			match self {$(
				Self::$Variant(r) => r.borrow_el(),
			)*}
		}

		pub fn as_trait_mut(&self) -> RefMut<dyn $Trait> {
			match self {$(
				Self::$Variant(r) => r.borrow_mut_el(),
			)*}
		}
	}
}}

decl_elem_ref! { enum (ElementRef, ElementRefWeak, ElementType) -> Any {
	// Atoms
	Accidental(Accidental),
	Articulation(Articulation),
	Chordline(Chordline),
	Hook(Hook),
	LedgerLine(LedgerLine),
	Note(Note),
	NoteDot(NoteDot),
	Notehead(Notehead),
	Stem(Stem),
	StemSlash(StemSlash),
	Symbol(Symbol),
	SymbolGroup(SymbolGroup),
	Text(Text),

	// Composites
	Measure(Measure),
	Part(Part),
	Staff(Staff),
	System(System),
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
	Arpeggio(Arpeggio),
	Beam(Beam),
	Line(Line),
	Slur(Slur),
	Tie(Tie),
	Tremolo(Tremolo),

	// Spanner Segments
	SlurSegment(SlurSegment),
	TieSegment(TieSegment),
	LineSegment(LineSegment),
}}

decl_elem_ref! { enum (MeasureRef, MeasureRefWeak, MeasureType) -> MeasureTrait {
	Measure(Measure),
	VBox(VBox),
	HBox(HBox),
}}

decl_elem_ref! { enum (SegmentRef, SegmentRefWeak, SegmentType) -> SegmentTrait {
	Barline(Barline),
	Chord(Chord),
	Clef(Clef),
	KeySig(KeySig),
	Rest(Rest),
	TimeSig(TimeSig),
}}

decl_elem_ref! { enum (SpannerRef, SpannerRefWeak, SpannerType) -> Any {
	Arpeggio(Arpeggio),
	Beam(Beam),
	Line(Line),
	Slur(Slur),
	Tie(Tie),
	Tremolo(Tremolo),
}}

decl_elem_ref! { enum (SpannerSegmentRef, SpannerSegmentRefWeak, SpannerSegmentType) -> Any {
	SlurSegment(SlurSegment),
	TieSegment(TieSegment),
	LineSegment(LineSegment),
}}

decl_elem_ref! { enum (ChordRef, ChordWeak, ChordType) -> Any {
	Chord(Chord),
	Rest(Rest),
}}