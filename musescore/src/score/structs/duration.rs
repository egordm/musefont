use super::*;
use crate::score::*;
use bitflags::_core::cmp::Ordering;

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DurationType {
	Long = 1,
	Breve = 2,
	Whole = 3,
	Half = 4,
	Quarter = 5,
	Eighth = 6,
	D16th = 7,
	D32nd = 8,
	D64th = 9,
	D128th = 10,
	D256th = 11,
	D512th = 12,
	D1024th = 13,
	Zero = 14,
	Measure = 15,
	Invalid = 16,
}

impl DurationType {
	pub fn ticks(&self) -> Fraction {
		match self {
			DurationType::Long => Fraction::new(4, 1),
			DurationType::Breve => Fraction::new(2, 1),
			DurationType::Whole => Fraction::new(1, 1),
			DurationType::Half => Fraction::new(1, 2),
			DurationType::Quarter => Fraction::new(1, 4),
			DurationType::Eighth => Fraction::new(1, 8),
			DurationType::D16th => Fraction::new(1, 16),
			DurationType::D32nd => Fraction::new(1, 32),
			DurationType::D64th => Fraction::new(1, 64),
			DurationType::D128th => Fraction::new(1, 128),
			DurationType::D256th => Fraction::new(1, 256),
			DurationType::D512th => Fraction::new(1, 512),
			DurationType::D1024th => Fraction::new(1, 1024),
			DurationType::Measure | DurationType::Zero => Fraction::new(0, 1),
			DurationType::Invalid => Fraction::new(-1, 1),
		}
	}

	pub fn note_head(&self) -> NoteheadType {
		match self {
			DurationType::Half => NoteheadType::Half,
			DurationType::Measure | DurationType::Whole => NoteheadType::Whole,
			DurationType::Long | DurationType::Breve => NoteheadType::Brevis,
			DurationType::Zero | DurationType::Invalid => NoteheadType::Quarter,
			_ => NoteheadType::Quarter
		}
	}

	pub fn has_stem(&self) -> bool {
		match self {
			DurationType::Breve | DurationType::Whole | DurationType::Zero | DurationType::Invalid => false,
			_ => true
		}
	}

	pub fn hook_type(&self) -> HookType {
		match self {
			DurationType::Eighth => HookType::Flag8thUp,
			DurationType::D16th => HookType::Flag16thUp,
			DurationType::D32nd => HookType::Flag32ndUp,
			DurationType::D64th => HookType::Flag64thUp,
			DurationType::D128th => HookType::Flag128thUp,
			DurationType::D256th => HookType::Flag256thUp,
			DurationType::D512th => HookType::Flag512thUp,
			DurationType::D1024th => HookType::Flag1024thUp,
			_ => HookType::None,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Duration {
	duration_type: DurationType,
	dots: u8,
}

impl Ord for Duration {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialOrd for Duration {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(match self.duration_type.cmp(&other.duration_type) {
			Ordering::Less => Ordering::Less,
			Ordering::Equal => self.dots.cmp(&other.dots),
			Ordering::Greater => Ordering::Greater,
		})
	}
}

impl Duration {
	pub fn new(duration_type: DurationType, dots: u8) -> Self { Self { duration_type, dots } }
}

impl Default for Duration {
	fn default() -> Self { Self { duration_type: DurationType::Quarter, dots: 0 } }
}

impl Duration {
	pub fn is_valid(&self) -> bool { self.duration_type != DurationType::Invalid }
	pub fn is_zero(&self) -> bool { self.duration_type != DurationType::Zero }
	pub fn is_measure(&self) -> bool { self.duration_type != DurationType::Measure }

	pub fn dots(&self) -> u8 { self.dots }
	pub fn set_dots(&mut self, v: u8) { self.dots = v }
	pub fn ticks(&self) -> Fraction {
		let mut ret = self.duration_type.ticks();
		for _ in 0..self.dots { ret *= Fraction::new(1, 2) }
		ret
	}
	pub fn head_type(&self) -> NoteheadType { self.duration_type.note_head() }
	pub fn has_stem(&self) -> bool { self.duration_type.has_stem() }

	pub fn hook_type(&self) -> HookType { self.duration_type.hook_type() }
	pub fn hook_count(&self) -> i32 { self.hook_type().count() }

	pub fn ty(&self) -> DurationType { self.duration_type }
}
