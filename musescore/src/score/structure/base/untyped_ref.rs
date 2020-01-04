use super::*;
use super::super::*;
use std::convert::TryInto;
use std::any::Any;

macro_rules! decl_elem_ref {{
	enum ($RefName: ident, $RefNameWeak: ident, $type_check: ident ->$RefTypeName: ident) -> $Trait:ident
	{
		$($Variant:ident($Type:ty)),* $(,)*
	}
} => {
	#[derive(Clone, Debug, Copy, Eq, PartialEq)]
	pub enum $RefTypeName {
		Invalid,
		$($Variant),*
	}

	pub fn $type_check(t: ElementType) -> bool {
		match t {
		    $(ElementType::$Variant)|* => true,
			_ => false,
		}
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
	$(impl TryInto<El<$Type>> for $RefName {
		type Error = ();
		fn try_into(self) -> Result<El<$Type>, Self::Error> {
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

decl_elem_ref! { enum (ElementRef, ElementRefWeak, is_element -> ElementType) -> Element {
	// Atoms
	Accidental(Accidental),
	Articulation(Articulation),
	Chordline(Chordline),
	Hook(Hook),
	LedgerLine(LedgerLine),
	Note(Note),
	NoteDot(NoteDot),
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
	LineSpanner(LineSpanner),
	Slur(Slur),
	Tie(Tie),
	Tremolo(Tremolo),

	// Spanner Segments
	SlurSegment(SlurSegment),
	TieSegment(TieSegment),
	LineSegment(LineSegment),
}}

decl_elem_ref! { enum (MeasureRef, MeasureRefWeak, is_measure -> MeasureType) -> MeasureTrait {
	Measure(Measure),
	VBox(VBox),
	HBox(HBox),
}}

decl_elem_ref! { enum (SegmentRef, SegmentRefWeak, is_segment -> SegmentType) -> SegmentTrait {
	Barline(Barline),
	Chord(Chord),
	Clef(Clef),
	KeySig(KeySig),
	Rest(Rest),
	TimeSig(TimeSig),
}}

decl_elem_ref! { enum (SpannerRef, SpannerRefWeak, is_spanner -> SpannerType) -> Any {
	Arpeggio(Arpeggio),
	Beam(Beam),
	LineSpanner(LineSpanner),
	Slur(Slur),
	Tie(Tie),
	Tremolo(Tremolo),
}}

decl_elem_ref! { enum (SpannerSegmentRef, SpannerSegmentRefWeak, is_spanner_segment -> SpannerSegmentType) -> Any {
	SlurSegment(SlurSegment),
	TieSegment(TieSegment),
	LineSegment(LineSegment),
}}

decl_elem_ref! { enum (ChordRef, ChordWeak, is_chord -> ChordType) -> Any {
	Chord(Chord),
	Rest(Rest),
}}