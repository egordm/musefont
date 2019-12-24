use crate::*;

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum DurationType {
	Long = 1,
	Breve = 2,
	Whole = 3,
	Half = 4,
	Quarter = 5,
	Eighth = 6,
	D16th = 7,
	D32th = 8,
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
			DurationType::D32th => Fraction::new(1, 32),
			DurationType::D64th => Fraction::new(1, 64),
			DurationType::D128th => Fraction::new(1, 128),
			DurationType::D256th => Fraction::new(1, 256),
			DurationType::D512th => Fraction::new(1, 512),
			DurationType::D1024th => Fraction::new(1, 1024),
			DurationType::Measure | DurationType::Zero => Fraction::new(0, 1),
			DurationType::Invalid => Fraction::new(-1, 1),
		}
	}

	pub fn note_head(&self) -> notehead::Type {
		match self {
			DurationType::Half => notehead::Type::Half,
			DurationType::Measure | DurationType::Whole => notehead::Type::Whole,
			DurationType::Long | DurationType::Breve => notehead::Type::Brevis,
			DurationType::Zero | DurationType::Invalid => notehead::Type::Quarter,
			_ => notehead::Type::Quarter
		}
	}

	pub fn has_stem(&self) -> bool {
		match self {
			DurationType::Breve | DurationType::Whole | DurationType::Zero | DurationType::Invalid => false,
			_ => true
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Duration {
	duration_type: DurationType,
	dots: u8,
}

impl Default for Duration {
	fn default() -> Self { Self { duration_type: DurationType::Quarter, dots: 0 } }
}

impl Duration {
	pub fn is_valid(&self) -> bool { self.duration_type != DurationType::Invalid }
	pub fn is_zero(&self) -> bool { self.duration_type != DurationType::Zero }
	pub fn is_measure(&self) -> bool { self.duration_type != DurationType::Measure }

	pub fn ticks(&self) -> Fraction {
		let mut ret = self.duration_type.ticks();
		for _ in 0..self.dots { ret *= Fraction::new(1, 2) }
		ret
	}
	pub fn note_head(&self) -> notehead::Type { self.duration_type.note_head() }
	pub fn has_stem(&self) -> bool { self.duration_type.has_stem() }

	pub fn ty(&self) -> DurationType { self.duration_type }
}