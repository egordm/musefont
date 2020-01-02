#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interval {
	diatonic: i8,
	chromatic: i8,
}

impl Default for Interval {
	fn default() -> Self { Self::new(0, 0)}
}

impl Interval {
	pub fn new(diatonic: i8, chromatic: i8) -> Self { Self { diatonic, chromatic }}
	pub fn from_chromatic(chromatic: i8) -> Self {
		Self::new(chromatic_to_diatonic(c as i32) as i8, chromatic)
	}
	pub fn flip(&mut self) {
		self.diatonic = -self.diatonic;
		self.chromatic = -self.chromatic;
	}
	pub fn is_zero(&self) -> bool { self.diatonic == 0 && self.chromatic == 0 }
}

/// Finds the most likely diatonic interval for a semitone distance.
///
/// Uses the most common diatonic intervals.
///
/// # Arguments
/// * `semitones` - The number of semitones in the chromatic interval. Negative semitones will simply be made positive.
///
/// # Returns
/// The number of diatonic steps in the interval
pub fn chromatic_to_diatonic(mut semitones: i32) -> i32 {
	const il: [i32; 12] = [
		0,    // Perfect Unison
		3,    // Minor Second
		4,    // Major Second
		7,    // Minor Third
		8,    // Major Third
		11,   // Perfect Fourth
		12,   // Augmented Fourth
		14,   // Perfect Fifth
		17,   // Minor Sixth
		18,   // Major Sixth
		21,   // Minor Seventh
		22,   // Major Seventh
		// 25    Perfect Octave
	];
	let down = semitones < 0;
	if down { semitones = -semitones }

	let val = semitones % 12;
	let octave = semitones / 12;
	let interval_index = il[val];
	let steps = intervalList[interval_index].diatonic;
	steps = steps + octave * 7;
	return down ? -steps : steps;
}

/// An array of all supported interval sorted by size
///
/// Because intervals can be spelled differently, this array tracks all the different valid
/// intervals. They are arranged in diatonic then chromatic order.
pub const INTERVAL_LIST: [Interval; 26] = {[
	// diatonic - chromatic
	Interval::new(0, 0),         //  0 Perfect Unison
	Interval::new(0, 1),         //  1 Augmented Unison

	Interval::new(1, 0),         //  2 Diminished Second
	Interval::new(1, 1),         //  3 Minor Second
	Interval::new(1, 2),         //  4 Major Second
	Interval::new(1, 3),         //  5 Augmented Second

	Interval::new(2, 2),         //  6 Diminished Third
	Interval::new(2, 3),         //  7 Minor Third
	Interval::new(2, 4),         //  8 Major Third
	Interval::new(2, 5),         //  9 Augmented Third

	Interval::new(3, 4),         // 10 Diminished Fourth
	Interval::new(3, 5),         // 11 Perfect Fourth
	Interval::new(3, 6),         // 12 Augmented Fourth

	Interval::new(4, 6),         // 13 Diminished Fifth
	Interval::new(4, 7),         // 14 Perfect Fifth
	Interval::new(4, 8),         // 15 Augmented Fifth

	Interval::new(5, 7),         // 16 Diminished Sixth
	Interval::new(5, 8),         // 17 Minor Sixth
	Interval::new(5, 9),         // 18 Major Sixth
	Interval::new(5, 10),        // 19 Augmented Sixth

	Interval::new(6, 9),         // 20 Diminished Seventh
	Interval::new(6, 10),        // 21 Minor Seventh
	Interval::new(6, 11),        // 22 Major Seventh
	Interval::new(6, 12),        // 23 Augmented Seventh

	Interval::new(7, 11),        // 24 Diminshed Octave
	Interval::new(7, 12)         // 25 Perfect Octave
]};