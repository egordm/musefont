use crate::score::*;
use crate::constants;
use once_cell::sync::Lazy;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupNode {
	/// tick position, division 32nd
	pos: i32,
	/// bits: cccc bbbb aaaa
	/// cc - 1/64  bb - 1/32  aa - 1/16
	/// bit pattern xxxx:
	/// 1 - start new beam
	/// 2 - start new 1/32 subbeam
	/// 3 - start new 1/64 subbeam
	action: u32,
}

impl GroupNode {
	pub fn new(pos: i32, action: u32) -> Self { Self { pos, action } }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Groups(Vec<GroupNode>);

impl Groups {
	pub fn new() -> Self { Self(Vec::new()) }

	pub fn is_empty(&self) -> bool { self.0.is_empty() }

	pub fn beam_mode(&self, tick: i32, d: DurationType) -> BeamMode {
		let shift = match d {
			DurationType::Eighth => 0,
			DurationType::D16th => 4,
			DurationType::D32nd => 8,
			_ => return BeamMode::Auto,
		};
		const DM: i32 = constants::DIVISION / 8;
		for node in self.0.iter() {
			if node.pos * DM < tick { continue }
			if node.pos * DM > tick { break }

			let action = (node.action >> shift) & 0xF;
			return match action {
				0 => BeamMode::Auto,
				1 => BeamMode::Begin,
				2 => BeamMode::Begin32,
				3 => BeamMode::Begin64,
				_ => BeamMode::Auto, // TODO: warn
			}
		}

		return BeamMode::Auto;
	}

	pub fn add_stop(&mut self, pos: i32, d: DurationType, bm: BeamMode) {
		let shift = match d {
			DurationType::Eighth => 0,
			DurationType::D16th => 4,
			DurationType::D32nd => 8,
			_ => return,
		};

		let action = match bm {
			BeamMode::Begin => 1,
			BeamMode::Begin32 => 2,
			BeamMode::Begin64 => 3,
			_ => return,
		};

		let pos = pos / 60;
		let action = action << shift;
		let mut index = 0;
		for (i, node) in self.0.iter_mut().enumerate() {
			index = i;
			if node.pos == pos {
				node.action = (node.action & !(0xF << shift)) | action;
				return;
			} else if node.pos > pos {
				break;
			}
		}

		self.0.insert(index, GroupNode { pos, action })
	}
}

impl Groups {
	pub fn end_beam(cr: &ChordRef, prev_cr: &Option<ChordRef>) -> BeamMode {
		cr.with(|cr: Ref<dyn ChordRestTrait>| {
			if cr.is_grace() || cr.beam_mode() != BeamMode::Auto {
				return cr.beam_mode();
			}
			assert!(cr.staff().is_some()); // TODO: Should assert or should handle?

			let d = cr.duration_type();
			let g = cr.staff().unwrap().borrow_el().group(&cr.time());
			let stretch = cr.staff().unwrap().borrow_el().timestretch(&cr.time());
			let time = cr.time() * stretch;
			let mut val = g.beam_mode(time.ticks(), d.ty());

			// context-dependent checks
			if val == BeamMode::Auto && !time.is_zero() {
				// if current or previous cr is in tuplet (but not both in same tuplet):
				// consider it as if this were next shorter duration
				if let Some(prev_cr) = prev_cr {
					if prev_cr.with(|prev_cr| cr.tuplet() != prev_cr.tuplet() && d == prev_cr.duration_type()) {
						if d.ty() >= DurationType::Eighth {
							val = g.beam_mode(time.ticks(), DurationType::D16th);
						} else if d.ty() == DurationType::D16th {
							val = g.beam_mode(time.ticks(), DurationType::D32nd);
						} else {
							val = g.beam_mode(time.ticks(), DurationType::D64th);
						}
					}
				}
				// if there is a hole between previous and current cr, break beam
				// exclude tuplets from this check; tick calculations can be unreliable
				// and they seem to be handled well anyhow
				if cr.voice() >= 0 {
					if let Some(prev_cr) = prev_cr {
						if prev_cr.with(|prev_cr| prev_cr.tuplet().is_some() && prev_cr.time() + prev_cr.actual_duration() < cr.time()) {
							val = BeamMode::Begin;
						}
					}
				}
			}

			return val;
		})
	}

	pub fn endings(f: &Fraction) -> &Groups {
		for g in unsafe { NOTEGROUPS.iter() } {
			if g.time_sig.identical(f) { return &g.endings; }
		}

		let mut g = NoteGroup::new(*f, Groups::new());
		let pos = match f.denominator {
			2 => 16,
			4 => 8,
			8 => 4,
			16 => 2,
			32 => 1,
			_ => 1
		};
		for i in 1..f.numerator {
			g.endings.0.push(GroupNode { pos: pos * i, action: 0x111 });
		}

		unsafe {
			NOTEGROUPS.push(g);
			return &NOTEGROUPS.last().unwrap().endings
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoteGroup {
	time_sig: Fraction,
	endings: Groups
}

impl NoteGroup {
	pub fn new(time_sig: Fraction, endings: Groups) -> Self { Self { time_sig, endings } }
}

static mut NOTEGROUPS: Lazy<Vec<NoteGroup>> = Lazy::new(|| {
	vec![
		NoteGroup {
			time_sig: Fraction::new(2, 2),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 272),
				GroupNode::new(12, 512),
				GroupNode::new(16, 273),
				GroupNode::new(20, 512),
				GroupNode::new(24, 272),
				GroupNode::new(28, 512),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(4, 4),
			endings: Groups(vec![
				GroupNode::new(4, 0x200),
				GroupNode::new(8, 0x110),
				GroupNode::new(12, 0x200),
				GroupNode::new(16, 0x111),
				GroupNode::new(20, 0x200),
				GroupNode::new(24, 0x110),
				GroupNode::new(28, 0x200),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(3, 4),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 273),
				GroupNode::new(12, 512),
				GroupNode::new(16, 273),
				GroupNode::new(20, 512),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(2, 4),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 273),
				GroupNode::new(12, 512),
				GroupNode::new(0, 0),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(6, 8),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 512),
				GroupNode::new(12, 273),
				GroupNode::new(16, 512),
				GroupNode::new(20, 512),
				GroupNode::new(0, 0),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(9, 8),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 512),
				GroupNode::new(12, 273),
				GroupNode::new(16, 512),
				GroupNode::new(20, 512),
				GroupNode::new(24, 273),
				GroupNode::new(18, 512),
				GroupNode::new(32, 512),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(12, 8),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 512),
				GroupNode::new(12, 273),
				GroupNode::new(16, 512),
				GroupNode::new(20, 512),
				GroupNode::new(24, 273),
				GroupNode::new(18, 512),
				GroupNode::new(32, 512),
				GroupNode::new(36, 273),
				GroupNode::new(40, 512),
				GroupNode::new(44, 512),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(6, 4),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 512),
				GroupNode::new(12, 512),
				GroupNode::new(16, 512),
				GroupNode::new(20, 512),
				GroupNode::new(24, 273),
				GroupNode::new(28, 512),
				GroupNode::new(32, 512),
				GroupNode::new(36, 512),
				GroupNode::new(40, 512),
				GroupNode::new(44, 512),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(3, 2),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 272),
				GroupNode::new(12, 512),
				GroupNode::new(16, 273),
				GroupNode::new(20, 512),
				GroupNode::new(24, 272),
				GroupNode::new(28, 512),
				GroupNode::new(32, 273),
				GroupNode::new(36, 512),
				GroupNode::new(40, 272),
				GroupNode::new(44, 512),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(5, 4),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 512),
				GroupNode::new(12, 512),
				GroupNode::new(16, 512),
				GroupNode::new(20, 512),
				GroupNode::new(24, 273),
				GroupNode::new(28, 512),
				GroupNode::new(32, 512),
				GroupNode::new(36, 512),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(7, 8),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 512),
				GroupNode::new(12, 273),
				GroupNode::new(16, 512),
				GroupNode::new(20, 273),
				GroupNode::new(24, 512),
			])
		},
		NoteGroup {
			time_sig: Fraction::new(5, 8),
			endings: Groups(vec![
				GroupNode::new(4, 512),
				GroupNode::new(8, 512),
				GroupNode::new(12, 273),
				GroupNode::new(16, 512),
			])
		},
	]
});