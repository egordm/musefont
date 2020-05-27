use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::score::StyleName::BeamMinLen;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct BeamMetricKey {
	/// The stem should be up
	pub up: bool,
	pub step1: i8,
	pub step2: i8,
}

#[derive(Copy, Clone, Debug)]
pub struct BeamMetric {
	/// stem len in 1/4 spatium units
	pub l: i8,
	/// beam slant in 1/4 spatium units
	pub s: i8,
}

impl Default for BeamMetric {
	fn default() -> Self { Self { l: 0, s: 0 } }
}

impl BeamMetric {
	pub fn new(l: i8, s: i8) -> Self { Self { l, s } }

	pub fn get(up: bool, step1: i8, step2: i8) -> BeamMetric {
		BEAM_METRICS.get(&BeamMetric::key(up, step1, step2)).cloned().unwrap_or_default()
	}

	fn key(up: bool, step1: i8, step2: i8) -> BeamMetricKey {
		BeamMetricKey { up, step1, step2 }
	}
}

const BEAM_METRICS: Lazy<HashMap<BeamMetricKey, BeamMetric>> = Lazy::new(|| {
	// TODO: step params are the line numbers. I should flip them here too.
	// Start from to top line and not from top to bottom
	let mut res = HashMap::new();

	// up  step1 step2 stemLen1 slant
	//                 (- up)   (- up)
	// =================================== C
	res.insert(BeamMetric::key(true, 10, 10), BeamMetric::new(-12, 0));
	res.insert(BeamMetric::key(false, 3, 3), BeamMetric::new(11, 0));
	res.insert(BeamMetric::key(true, 3, 3), BeamMetric::new(-11, 0));

	res.insert(BeamMetric::key(true, 10, 9), BeamMetric::new(-12, -1));
	res.insert(BeamMetric::key(true, 10, 8), BeamMetric::new(-12, -4));
	res.insert(BeamMetric::key(true, 10, 7), BeamMetric::new(-12, -5));
	res.insert(BeamMetric::key(true, 10, 6), BeamMetric::new(-15, -5));
	res.insert(BeamMetric::key(true, 10, 5), BeamMetric::new(-16, -5));
	res.insert(BeamMetric::key(true, 10, 4), BeamMetric::new(-20, -4));
	res.insert(BeamMetric::key(true, 10, 3), BeamMetric::new(-20, -5));

	res.insert(BeamMetric::key(true, 10, 11), BeamMetric::new(-12, 1));
	res.insert(BeamMetric::key(true, 10, 12), BeamMetric::new(-13, 2));      // F
	res.insert(BeamMetric::key(true, 10, 13), BeamMetric::new(-13, 2));
	res.insert(BeamMetric::key(true, 10, 14), BeamMetric::new(-13, 2));
	res.insert(BeamMetric::key(true, 10, 15), BeamMetric::new(-13, 2));

	res.insert(BeamMetric::key(true, 3, 4), BeamMetric::new(-11, 1));
	res.insert(BeamMetric::key(true, 3, 5), BeamMetric::new(-11, 2));
	res.insert(BeamMetric::key(true, 3, 6), BeamMetric::new(-11, 4));
	res.insert(BeamMetric::key(true, 3, 7), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 3, 8), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 3, 9), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 3, 10), BeamMetric::new(-11, 5));

	res.insert(BeamMetric::key(false, -4, -3), BeamMetric::new(15, 1));
	res.insert(BeamMetric::key(false, -4, -2), BeamMetric::new(15, 2));
	res.insert(BeamMetric::key(false, -4, -1), BeamMetric::new(15, 2));
	res.insert(BeamMetric::key(false, -4, 0), BeamMetric::new(15, 5));
	res.insert(BeamMetric::key(false, -4, 1), BeamMetric::new(16, 5));
	res.insert(BeamMetric::key(false, -4, 2), BeamMetric::new(20, 4));
	res.insert(BeamMetric::key(false, -4, 3), BeamMetric::new(20, 5));

	res.insert(BeamMetric::key(false, 3, 4), BeamMetric::new(13, 1));
	res.insert(BeamMetric::key(false, 3, 5), BeamMetric::new(13, 2));
	res.insert(BeamMetric::key(false, 3, 6), BeamMetric::new(14, 4));
	res.insert(BeamMetric::key(false, 3, 7), BeamMetric::new(14, 4));
	res.insert(BeamMetric::key(false, 3, 8), BeamMetric::new(14, 6));

	res.insert(BeamMetric::key(false, 3, 2), BeamMetric::new(11, -1));
	res.insert(BeamMetric::key(false, 3, 1), BeamMetric::new(11, -2));
	res.insert(BeamMetric::key(false, 3, 0), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 3, -1), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 3, -2), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 3, -3), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 3, -4), BeamMetric::new(11, -5));

// =================================== D
	res.insert(BeamMetric::key(true, 9, 9), BeamMetric::new(-13, 0));
	res.insert(BeamMetric::key(false, 2, 2), BeamMetric::new(12, 0));
	res.insert(BeamMetric::key(true, 2, 2), BeamMetric::new(-11, 0));

	res.insert(BeamMetric::key(true, 9, 8), BeamMetric::new(-13, -1));
	res.insert(BeamMetric::key(true, 9, 7), BeamMetric::new(-13, -2));
	res.insert(BeamMetric::key(true, 9, 6), BeamMetric::new(-13, -5));
	res.insert(BeamMetric::key(true, 9, 5), BeamMetric::new(-14, -5));
	res.insert(BeamMetric::key(true, 9, 4), BeamMetric::new(-16, -6));
	res.insert(BeamMetric::key(true, 9, 3), BeamMetric::new(-17, -5));
	res.insert(BeamMetric::key(true, 9, 2), BeamMetric::new(-17, -8));

	res.insert(BeamMetric::key(true, 9, 10), BeamMetric::new(-11, 1));
	res.insert(BeamMetric::key(true, 9, 11), BeamMetric::new(-11, 2));
	res.insert(BeamMetric::key(true, 9, 12), BeamMetric::new(-11, 2));
	res.insert(BeamMetric::key(true, 9, 13), BeamMetric::new(-11, 2));
	res.insert(BeamMetric::key(true, 9, 14), BeamMetric::new(-11, 2));
	res.insert(BeamMetric::key(true, 9, 15), BeamMetric::new(-11, 2));

	res.insert(BeamMetric::key(true, 2, 3), BeamMetric::new(-12, 1));
	res.insert(BeamMetric::key(true, 2, 4), BeamMetric::new(-12, 2));
	res.insert(BeamMetric::key(true, 2, 5), BeamMetric::new(-12, 4));
	res.insert(BeamMetric::key(true, 2, 6), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 2, 7), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 2, 8), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 2, 9), BeamMetric::new(-12, 8));

	res.insert(BeamMetric::key(false, -5, -4), BeamMetric::new(16, 2));
	res.insert(BeamMetric::key(false, -5, -3), BeamMetric::new(16, 2));
	res.insert(BeamMetric::key(false, -5, -2), BeamMetric::new(17, 2));
	res.insert(BeamMetric::key(false, -5, -1), BeamMetric::new(17, 2));
	res.insert(BeamMetric::key(false, -5, 0), BeamMetric::new(18, 4));
	res.insert(BeamMetric::key(false, -5, 1), BeamMetric::new(18, 5));
	res.insert(BeamMetric::key(false, -5, 2), BeamMetric::new(21, 5));

	res.insert(BeamMetric::key(false, 2, 3), BeamMetric::new(12, 1));
	res.insert(BeamMetric::key(false, 2, 4), BeamMetric::new(12, 4));
	res.insert(BeamMetric::key(false, 2, 5), BeamMetric::new(13, 4));  // F
	res.insert(BeamMetric::key(false, 2, 6), BeamMetric::new(15, 5));
	res.insert(BeamMetric::key(false, 2, 7), BeamMetric::new(15, 6));
	res.insert(BeamMetric::key(false, 2, 8), BeamMetric::new(16, 8));
	res.insert(BeamMetric::key(false, 2, 9), BeamMetric::new(16, 8));

	res.insert(BeamMetric::key(false, 2, 1), BeamMetric::new(12, -1));
	res.insert(BeamMetric::key(false, 2, 0), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 2, -1), BeamMetric::new(12, -5));
	res.insert(BeamMetric::key(false, 2, -2), BeamMetric::new(12, -5));
	res.insert(BeamMetric::key(false, 2, -3), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 2, -4), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 2, -5), BeamMetric::new(12, -5));

// =================================== E
	res.insert(BeamMetric::key(true, 8, 8), BeamMetric::new(-12, 0));
	res.insert(BeamMetric::key(false, 1, 1), BeamMetric::new(13, 0));
	res.insert(BeamMetric::key(true, 1, 1), BeamMetric::new(-12, 0));

	res.insert(BeamMetric::key(true, 8, 7), BeamMetric::new(-12, -1));
	res.insert(BeamMetric::key(true, 8, 6), BeamMetric::new(-12, -4));
	res.insert(BeamMetric::key(true, 8, 5), BeamMetric::new(-12, -5));
	res.insert(BeamMetric::key(true, 8, 4), BeamMetric::new(-15, -5));
	res.insert(BeamMetric::key(true, 8, 3), BeamMetric::new(-16, -5));
	res.insert(BeamMetric::key(true, 8, 2), BeamMetric::new(-17, -6));
	res.insert(BeamMetric::key(true, 8, 1), BeamMetric::new(-19, -6));

	res.insert(BeamMetric::key(true, 15, 11), BeamMetric::new(-21, -1));
	res.insert(BeamMetric::key(true, 15, 10), BeamMetric::new(-21, -1));
	res.insert(BeamMetric::key(true, 15, 9), BeamMetric::new(-21, -4));
	res.insert(BeamMetric::key(true, 15, 8), BeamMetric::new(-21, -5));

	res.insert(BeamMetric::key(true, 1, 8), BeamMetric::new(-11, 6));
	res.insert(BeamMetric::key(true, 1, 7), BeamMetric::new(-11, 6));
	res.insert(BeamMetric::key(true, 1, 6), BeamMetric::new(-12, 6));

	res.insert(BeamMetric::key(true, 8, 9), BeamMetric::new(-12, 1));
	res.insert(BeamMetric::key(true, 8, 10), BeamMetric::new(-12, 4));
	res.insert(BeamMetric::key(true, 8, 11), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 8, 12), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 8, 13), BeamMetric::new(-12, 4));
	res.insert(BeamMetric::key(true, 8, 14), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 8, 15), BeamMetric::new(-12, 5));

	res.insert(BeamMetric::key(false, 1, 0), BeamMetric::new(11, -1));
	res.insert(BeamMetric::key(false, 1, -1), BeamMetric::new(11, -2));
	res.insert(BeamMetric::key(false, 1, -2), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 1, -3), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 1, -4), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 1, -5), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 1, -6), BeamMetric::new(11, -5));

	res.insert(BeamMetric::key(false, 1, 2), BeamMetric::new(13, 1));
	res.insert(BeamMetric::key(false, 1, 3), BeamMetric::new(13, 2));
	res.insert(BeamMetric::key(false, 1, 4), BeamMetric::new(13, 5));
	res.insert(BeamMetric::key(false, 1, 5), BeamMetric::new(14, 5));
	res.insert(BeamMetric::key(false, 1, 6), BeamMetric::new(15, 5));
	res.insert(BeamMetric::key(false, 1, 7), BeamMetric::new(17, 5));
	res.insert(BeamMetric::key(false, 1, 8), BeamMetric::new(17, 8));

	res.insert(BeamMetric::key(false, -6, -2), BeamMetric::new(19, 2));
	res.insert(BeamMetric::key(false, -6, -1), BeamMetric::new(19, 4));
	res.insert(BeamMetric::key(false, -6, 0), BeamMetric::new(20, 4));
	res.insert(BeamMetric::key(false, -6, 1), BeamMetric::new(20, 5));

	res.insert(BeamMetric::key(false, 8, 3), BeamMetric::new(9, -6));
	res.insert(BeamMetric::key(false, 8, 2), BeamMetric::new(12, -8));
	res.insert(BeamMetric::key(false, 8, 1), BeamMetric::new(12, -8));

// =================================== F
	res.insert(BeamMetric::key(true, 7, 7), BeamMetric::new(-13, 0));      //F
	res.insert(BeamMetric::key(false, 0, 0), BeamMetric::new(12, 0));
	res.insert(BeamMetric::key(false, 7, 7), BeamMetric::new(12, 0));

	res.insert(BeamMetric::key(true, 7, 6), BeamMetric::new(-13, -1));
	res.insert(BeamMetric::key(true, 7, 5), BeamMetric::new(-13, -2));
	res.insert(BeamMetric::key(true, 7, 4), BeamMetric::new(-13, -5));
	res.insert(BeamMetric::key(true, 7, 3), BeamMetric::new(-14, -5));
	res.insert(BeamMetric::key(true, 7, 2), BeamMetric::new(-15, -6));
	res.insert(BeamMetric::key(true, 7, 1), BeamMetric::new(-17, -6));
	res.insert(BeamMetric::key(true, 7, 0), BeamMetric::new(-18, -8));

	res.insert(BeamMetric::key(true, 14, 10), BeamMetric::new(-19, -2));
	res.insert(BeamMetric::key(true, 14, 9), BeamMetric::new(-19, -2));
	res.insert(BeamMetric::key(true, 14, 8), BeamMetric::new(-20, -4));
	res.insert(BeamMetric::key(true, 14, 7), BeamMetric::new(-20, -5));

	res.insert(BeamMetric::key(true, 0, 5), BeamMetric::new(-9, 6));
	res.insert(BeamMetric::key(true, 0, 6), BeamMetric::new(-12, 8));
	res.insert(BeamMetric::key(true, 0, 7), BeamMetric::new(-12, 8));

	res.insert(BeamMetric::key(true, 7, 8), BeamMetric::new(-11, 1));
	res.insert(BeamMetric::key(true, 7, 9), BeamMetric::new(-11, 2));
	res.insert(BeamMetric::key(true, 7, 10), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 7, 11), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 7, 12), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 7, 13), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 7, 14), BeamMetric::new(-11, 5));

	res.insert(BeamMetric::key(false, 0, -1), BeamMetric::new(12, -1));
	res.insert(BeamMetric::key(false, 0, -2), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 0, -3), BeamMetric::new(12, -5));
	res.insert(BeamMetric::key(false, 0, -4), BeamMetric::new(12, -5));
	res.insert(BeamMetric::key(false, 0, -5), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 0, -6), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 0, -7), BeamMetric::new(12, -4));

	res.insert(BeamMetric::key(false, 0, 1), BeamMetric::new(12, 1));
	res.insert(BeamMetric::key(false, 0, 2), BeamMetric::new(12, 4));
	res.insert(BeamMetric::key(false, 0, 3), BeamMetric::new(12, 5));
	res.insert(BeamMetric::key(false, 0, 4), BeamMetric::new(15, 5));
	res.insert(BeamMetric::key(false, 0, 5), BeamMetric::new(16, 5));
	res.insert(BeamMetric::key(false, 0, 6), BeamMetric::new(17, 5));
	res.insert(BeamMetric::key(false, 0, 7), BeamMetric::new(19, 6));

	res.insert(BeamMetric::key(false, -7, -3), BeamMetric::new(21, 2));
	res.insert(BeamMetric::key(false, -7, -2), BeamMetric::new(21, 2));
	res.insert(BeamMetric::key(false, -7, -1), BeamMetric::new(21, 2));
	res.insert(BeamMetric::key(false, -7, 0), BeamMetric::new(22, 4));

	res.insert(BeamMetric::key(false, 7, 2), BeamMetric::new(12, -6));
	res.insert(BeamMetric::key(false, 7, 1), BeamMetric::new(11, -6));
	res.insert(BeamMetric::key(false, 7, 0), BeamMetric::new(11, -6));

// =================================== G
	res.insert(BeamMetric::key(true, 6, 6), BeamMetric::new(-12, 0));
	res.insert(BeamMetric::key(false, -1, -1), BeamMetric::new(13, 0));
	res.insert(BeamMetric::key(false, 6, 6), BeamMetric::new(11, 0));

	res.insert(BeamMetric::key(true, 6, 5), BeamMetric::new(-12, -1));
	res.insert(BeamMetric::key(true, 6, 4), BeamMetric::new(-12, -4));
	res.insert(BeamMetric::key(true, 6, 3), BeamMetric::new(-13, -4));
	res.insert(BeamMetric::key(true, 6, 2), BeamMetric::new(-15, -5));
	res.insert(BeamMetric::key(true, 6, 1), BeamMetric::new(-13, -7));
	res.insert(BeamMetric::key(true, 6, 0), BeamMetric::new(-16, -8));
	res.insert(BeamMetric::key(true, 6, -1), BeamMetric::new(-16, -8));

	res.insert(BeamMetric::key(true, 13, 10), BeamMetric::new(-17, -2));
	res.insert(BeamMetric::key(true, 13, 9), BeamMetric::new(-17, -2));
	res.insert(BeamMetric::key(true, 13, 8), BeamMetric::new(-18, -4));
	res.insert(BeamMetric::key(true, 13, 7), BeamMetric::new(-18, -5));
	res.insert(BeamMetric::key(true, 13, 6), BeamMetric::new(-21, -5));

	res.insert(BeamMetric::key(true, -1, 6), BeamMetric::new(-10, 8));

	res.insert(BeamMetric::key(true, 6, 7), BeamMetric::new(-12, 1));
	res.insert(BeamMetric::key(true, 6, 8), BeamMetric::new(-12, 4));
	res.insert(BeamMetric::key(true, 6, 9), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 6, 10), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 6, 11), BeamMetric::new(-12, 4));
	res.insert(BeamMetric::key(true, 6, 12), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 6, 13), BeamMetric::new(-12, 5));

	res.insert(BeamMetric::key(false, -1, -2), BeamMetric::new(11, -1));
	res.insert(BeamMetric::key(false, -1, -3), BeamMetric::new(11, -2));
	res.insert(BeamMetric::key(false, -1, -4), BeamMetric::new(11, -2));
	res.insert(BeamMetric::key(false, -1, -5), BeamMetric::new(11, -2));
	res.insert(BeamMetric::key(false, -1, -6), BeamMetric::new(11, -2));
	res.insert(BeamMetric::key(false, -1, -7), BeamMetric::new(11, -2));

	res.insert(BeamMetric::key(false, -1, 0), BeamMetric::new(13, 1));
	res.insert(BeamMetric::key(false, -1, 1), BeamMetric::new(13, 2));
	res.insert(BeamMetric::key(false, -1, 2), BeamMetric::new(13, 5));
	res.insert(BeamMetric::key(false, -1, 3), BeamMetric::new(14, 5));
	res.insert(BeamMetric::key(false, -1, 4), BeamMetric::new(17, 6));
	res.insert(BeamMetric::key(false, -1, 5), BeamMetric::new(18, 5));
	res.insert(BeamMetric::key(false, -1, 6), BeamMetric::new(18, 8));

	res.insert(BeamMetric::key(false, 6, 5), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 6, 4), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 6, 3), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 6, 2), BeamMetric::new(12, -6));
	res.insert(BeamMetric::key(false, 6, 1), BeamMetric::new(11, -6));
	res.insert(BeamMetric::key(false, 6, 0), BeamMetric::new(12, -7));
	res.insert(BeamMetric::key(false, 6, -1), BeamMetric::new(12, -8));

// =================================== A
	res.insert(BeamMetric::key(true, 5, 5), BeamMetric::new(-11, 0));
	res.insert(BeamMetric::key(false, -2, -2), BeamMetric::new(12, 0));
	res.insert(BeamMetric::key(false, 5, 5), BeamMetric::new(11, 0));

	res.insert(BeamMetric::key(true, 5, 4), BeamMetric::new(-13, -1));
	res.insert(BeamMetric::key(true, 5, 3), BeamMetric::new(-13, -2));
	res.insert(BeamMetric::key(true, 5, 2), BeamMetric::new(-14, -4));
	res.insert(BeamMetric::key(true, 5, 1), BeamMetric::new(-15, -4));
	res.insert(BeamMetric::key(true, 5, 0), BeamMetric::new(-15, -6));

	res.insert(BeamMetric::key(true, 12, 11), BeamMetric::new(-15, -1));
	res.insert(BeamMetric::key(true, 12, 10), BeamMetric::new(-15, -2));
	res.insert(BeamMetric::key(true, 12, 9), BeamMetric::new(-15, -2));
	res.insert(BeamMetric::key(true, 12, 8), BeamMetric::new(-15, -5));
	res.insert(BeamMetric::key(true, 12, 7), BeamMetric::new(-16, -5));
	res.insert(BeamMetric::key(true, 12, 6), BeamMetric::new(-20, -4));
	res.insert(BeamMetric::key(true, 12, 5), BeamMetric::new(-20, -5));

	res.insert(BeamMetric::key(true, 5, 6), BeamMetric::new(-11, 1));
	res.insert(BeamMetric::key(true, 5, 7), BeamMetric::new(-11, 2));
	res.insert(BeamMetric::key(true, 5, 8), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 5, 9), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 5, 10), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 5, 11), BeamMetric::new(-11, 5));
	res.insert(BeamMetric::key(true, 5, 12), BeamMetric::new(-11, 5));

	res.insert(BeamMetric::key(false, -2, -1), BeamMetric::new(12, 1));
	res.insert(BeamMetric::key(false, -2, 0), BeamMetric::new(12, 4));
	res.insert(BeamMetric::key(false, -2, 1), BeamMetric::new(12, 5));
	res.insert(BeamMetric::key(false, -2, 2), BeamMetric::new(15, 5));
	res.insert(BeamMetric::key(false, -2, 3), BeamMetric::new(16, 5));
	res.insert(BeamMetric::key(false, -2, 4), BeamMetric::new(20, 4));
	res.insert(BeamMetric::key(false, -2, 5), BeamMetric::new(20, 5));

	res.insert(BeamMetric::key(false, -2, -3), BeamMetric::new(12, -1));
	res.insert(BeamMetric::key(false, -2, -4), BeamMetric::new(13, -2));
	res.insert(BeamMetric::key(false, -2, -5), BeamMetric::new(13, -2));
	res.insert(BeamMetric::key(false, -2, -6), BeamMetric::new(13, -2));
	res.insert(BeamMetric::key(false, -2, -7), BeamMetric::new(13, -2));

	res.insert(BeamMetric::key(false, 5, 4), BeamMetric::new(11, -1));
	res.insert(BeamMetric::key(false, 5, 3), BeamMetric::new(11, -2));
	res.insert(BeamMetric::key(false, 5, 2), BeamMetric::new(11, -4));
	res.insert(BeamMetric::key(false, 5, 1), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 5, 0), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 5, -1), BeamMetric::new(11, -5));
	res.insert(BeamMetric::key(false, 5, -2), BeamMetric::new(11, -5));

// =================================== B
	res.insert(BeamMetric::key(true, 4, 4), BeamMetric::new(-12, 0));
	res.insert(BeamMetric::key(true, 11, 11), BeamMetric::new(-13, 0));
	res.insert(BeamMetric::key(false, 4, 4), BeamMetric::new(12, 0));
	res.insert(BeamMetric::key(false, -3, -3), BeamMetric::new(13, 0));

	res.insert(BeamMetric::key(true, 11, 10), BeamMetric::new(-13, -1));
	res.insert(BeamMetric::key(true, 11, 9), BeamMetric::new(-13, -2));
	res.insert(BeamMetric::key(true, 11, 8), BeamMetric::new(-13, -5));
	res.insert(BeamMetric::key(true, 11, 7), BeamMetric::new(-14, -5));
	res.insert(BeamMetric::key(true, 11, 6), BeamMetric::new(-18, -4));
	res.insert(BeamMetric::key(true, 11, 5), BeamMetric::new(-18, -5));
	res.insert(BeamMetric::key(true, 11, 4), BeamMetric::new(-21, -5));

	res.insert(BeamMetric::key(true, 4, 3), BeamMetric::new(-12, -1));
	res.insert(BeamMetric::key(true, 4, 2), BeamMetric::new(-12, -4));
	res.insert(BeamMetric::key(true, 4, 1), BeamMetric::new(-14, -4));
	res.insert(BeamMetric::key(true, 4, 0), BeamMetric::new(-16, -4));

	res.insert(BeamMetric::key(true, 11, 12), BeamMetric::new(-14, 1));
	res.insert(BeamMetric::key(true, 11, 13), BeamMetric::new(-14, 1));
	res.insert(BeamMetric::key(true, 11, 14), BeamMetric::new(-14, 1));
	res.insert(BeamMetric::key(true, 11, 15), BeamMetric::new(-15, 2));
	res.insert(BeamMetric::key(true, 11, 16), BeamMetric::new(-15, 2));

	res.insert(BeamMetric::key(true, 4, 5), BeamMetric::new(-12, 1));
	res.insert(BeamMetric::key(true, 4, 6), BeamMetric::new(-12, 4));
	res.insert(BeamMetric::key(true, 4, 7), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 4, 8), BeamMetric::new(-12, 5));
	res.insert(BeamMetric::key(true, 4, 9), BeamMetric::new(-13, 6));
	res.insert(BeamMetric::key(true, 4, 10), BeamMetric::new(-12, 4));
	res.insert(BeamMetric::key(true, 4, 11), BeamMetric::new(-12, 5));

	res.insert(BeamMetric::key(false, 4, 3), BeamMetric::new(12, -1));
	res.insert(BeamMetric::key(false, 4, 2), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 4, 1), BeamMetric::new(12, -5));
	res.insert(BeamMetric::key(false, 4, 0), BeamMetric::new(12, -5));
	res.insert(BeamMetric::key(false, 4, -1), BeamMetric::new(13, -6));
	res.insert(BeamMetric::key(false, 4, -2), BeamMetric::new(12, -4));
	res.insert(BeamMetric::key(false, 4, -3), BeamMetric::new(12, -5));

	res.insert(BeamMetric::key(false, 4, 5), BeamMetric::new(12, 1));
	res.insert(BeamMetric::key(false, 4, 6), BeamMetric::new(12, 4));

	res.insert(BeamMetric::key(false, -3, -4), BeamMetric::new(14, -1));
	res.insert(BeamMetric::key(false, -3, -5), BeamMetric::new(14, -1));
	res.insert(BeamMetric::key(false, -3, -6), BeamMetric::new(14, -1));
	res.insert(BeamMetric::key(false, -3, -7), BeamMetric::new(15, -2));
	res.insert(BeamMetric::key(false, -3, -8), BeamMetric::new(15, -2));
	res.insert(BeamMetric::key(false, -3, -9), BeamMetric::new(15, -2));

	res.insert(BeamMetric::key(false, -3, -2), BeamMetric::new(13, 1));
	res.insert(BeamMetric::key(false, -3, -1), BeamMetric::new(13, 2));
	res.insert(BeamMetric::key(false, -3, 0), BeamMetric::new(13, 5));
	res.insert(BeamMetric::key(false, -3, 1), BeamMetric::new(14, 5));
	res.insert(BeamMetric::key(false, -3, 2), BeamMetric::new(18, 4));
	res.insert(BeamMetric::key(false, -3, 3), BeamMetric::new(18, 5));
	res.insert(BeamMetric::key(false, -3, 4), BeamMetric::new(21, 5));
	res
});