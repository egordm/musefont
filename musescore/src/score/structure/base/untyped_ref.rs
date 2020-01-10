use super::*;
use super::super::*;
use std::convert::{TryInto, TryFrom};
use std::any::Any;

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! conv_elem_ref {
{ enum ElementRef { $($Variant:ident($Type:ty)),* $(,)* } } => {

};

{ enum $RefName: ident { $($Variant:ident($Type:ty)),* $(,)* } } => {
	impl TryFrom<ElementRef> for $RefName {
		type Error = ();
		fn try_from(value: ElementRef) -> Result<$RefName, Self::Error> {
			match value {
				$(ElementRef::$Variant(r) => Ok($RefName::$Variant(r)),)*
				_ => Err(())
			}
		}
	}
}
}

macro_rules! decl_elem_ref {{
	enum ($RefName: ident, $RefNameWeak: ident, $type_check: ident ->$RefTypeName: ident) -> $Trait:ident
	{
		$($Variant:ident($Type:ty)),* $(,)*
	}
} => {
	#[repr(u8)]
	#[derive(Clone, Debug, Copy, Eq, PartialEq)]
	pub enum $RefTypeName {
		Invalid,
		$($Variant),*
	}

	impl TryFrom<usize> for $RefTypeName {
		type Error = ();

		fn try_from(value: usize) -> Result<Self, Self::Error> {
			if value < Self::count() {
				Ok(unsafe { std::mem::transmute(value as u8) })
			} else {
				Err(())
			}
		}
	}

	impl $RefTypeName {
		pub const fn count() -> usize {
			count!($($Variant)*)
		}
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

	conv_elem_ref! {
		enum $RefName {
			$($Variant($Type)),*
		}
	}

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
	Spacer(Spacer),
	StaffLines(StaffLines),
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
	Segment(Segment),
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

decl_elem_ref! { enum (AtomRef, AtomRefWeak, is_atom -> AtomType) -> AtomTrait {
	Accidental(Accidental),
	Articulation(Articulation),
	Chordline(Chordline),
	Hook(Hook),
	LedgerLine(LedgerLine),
	Note(Note),
	NoteDot(NoteDot),
	Spacer(Spacer),
	StaffLines(StaffLines),
	Stem(Stem),
	StemSlash(StemSlash),
	Symbol(Symbol),
	SymbolGroup(SymbolGroup),
	Text(Text),
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
	Segment(Segment),
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

decl_elem_ref! { enum (ChordRef, ChordWeak, is_chord -> ChordType) -> ChordRestTrait {
	Chord(Chord),
	Rest(Rest),
}}

decl_elem_ref! { enum (DurationElementRef, DurationElementWeak, is_duration_element -> DurationElementType) -> DurationElement {
	Chord(Chord),
	Rest(Rest),
	Tuplet(Tuplet),
}}