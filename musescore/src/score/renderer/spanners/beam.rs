use log::{warn};
use crate::*;
use crate::score::*;
use crate::drawing::{PainterRef, Instruction, Path, Segment as Seg};
use std::collections::HashMap;
use num_traits::real::Real;
use std::convert::TryInto;
use crate::score::NoteheadGroup::B;

pub struct BeamRenderer {}

impl Renderer<Beam> for BeamRenderer {
	fn layout(e: El<Beam>) {
		let system: Option<El<System>> = e.borrow_el().elements().next()
			.and_then(|e: _| e.as_trait().system());
		e.borrow_mut_el().set_parent_el(system.clone());

		// TODO: If measures from diff systems. Split in fragments
		let crl: Vec<ChordRef> = e.borrow_el().elements().cloned().collect();
		let n = 0;
		if !crl.is_empty() {
			let sv = if n == 0 { SpannerSegmentVariant::Begin } else { SpannerSegmentVariant::End };
			if e.borrow_el().fragments().len() < n + 1 {
				e.borrow_mut_el().fragments_mut().push(BeamFragment::default());
			}
			Self::layout_fragment(e.clone(), &crl, sv, n);
			let lw2 = e.with(|e| e.score().point(e.style().value_spatium(StyleName::BeamWidth)) * 0.5 * e.scale());

			// Update the bounding box
			e.with_mut(|mut e| {
				let mut bbox = RectF::default();
				for bs in e.beam_segments() {
					let mut a = PolygonF::new();
					a.push(Point2F::new(bs.x1(), bs.x1()));
					a.push(Point2F::new(bs.x2(), bs.y2()));
					a.push(Point2F::new(bs.x2(), bs.y2()));
					a.push(Point2F::new(bs.x1(), bs.x1()));
					let r = a.bbox().adjust(Point2F::new(0., -lw2), Point2F::new(0., lw2));
					bbox = bbox.union(&r);
				}
				e.set_bbox(bbox);
			});
		}
	}

	fn render(e: El<Beam>, state: &mut RendererState, painter: PainterRef) {
		e.with(|e| {
			if e.beam_segments().is_empty() {
				return;
			}
			painter.set_color(crate::COLOR_BLACK);
			let lw2 = e.score().point(e.style().value_spatium(StyleName::BeamWidth)) * 0.5 * e.scale();

			// make beam thickness independent of slant
			// (expression can be simplified?)
			let bs = e.beam_segments().first().unwrap();
			let mut d = (bs.y2() - bs.y1()).abs() / (bs.x2() - bs.x1());
			if e.beam_segments().len() > 1 && d > std::f32::consts::PI / 6. {
				d = std::f32::consts::PI / 6.;
			}
			let ww = lw2 / (constants::PI_HALF - d.atan()).sin();
			for bs in e.beam_segments() {
				painter.set_color(crate::COLOR_BLACK);
				let path = Path::new().set_fill(true)
					.move_to(Vec2F::new(bs.x1(), bs.y1() - ww))
					.add_segment(Seg::Line(Vec2F::new(bs.x2(), bs.y2() - ww)))
					.add_segment(Seg::Line(Vec2F::new(bs.x2(), bs.y2() + ww)))
					.add_segment(Seg::Line(Vec2F::new(bs.x1(), bs.y1() + ww)));

				painter.draw(Instruction::Path(path));
			}

			/*if state.debug() {
				painter.set_color(crate::COLOR_GREEN);
				painter.draw(drawing::Instruction::Rect(e.bbox().translate(e.pos().to_vector()), 1.));
				painter.set_color(crate::COLOR_BLUE);
				painter.draw(drawing::Instruction::Point(e.pos(), 2.));
				painter.set_color(crate::COLOR_BLACK);
			}*/
		});
	}
}

impl BeamRenderer {
	/// Whether beam between given notes should be slanted
	pub fn zero_slant(e: El<Beam>, cl: &Vec<ChordRef>) -> bool {
		// TODO: take into account alternatve line counting
		e.with(|e| {
			if e.has_no_slope() || cl.len() < 2 {
				return true;
			}
			if cl.len() == 2 && (ChordType::from_opt(cl.first()) == ChordType::Rest
				|| ChordType::from_opt(cl.last()) == ChordType::Rest) {
				return true;
			}

			let l1: Line = cl.first().unwrap().as_trait().line();
			let le: Line = cl.last().unwrap().as_trait().line();

			// look for some pattern
			if cl.len() == 4 {
				let l2: Line = cl[1].as_trait().line();
				let l3: Line = cl[2].as_trait().line();

				if (l1 < le) && (l2 > l1) && (l2 > l3) && (l3 > le) {
					return true;
				}
				if (l1 == l3) && (l2 == le) {
					return true;
				}
			} else if cl.len() == 6 {
				let l2: Line = cl[1].as_trait().line();
				let l3: Line = cl[2].as_trait().line();
				let l4: Line = cl[3].as_trait().line();
				let l5: Line = cl[4].as_trait().line();
				if (l2 > l1) && (l3 > l2) && (l1 == l4) && (l2 == l5) && (l3 == le) {
					return true;
				}
			}

			// Concave beams have a slope of 0
			let mut same_line = true;
			// TODO: set slope = 0.0 !!

			if cl.len() >= 3 {
				let l4: Line = cl[1].as_trait().line_dir(e.up());
				for ci in cl.iter().skip(1)
					.filter(|ci| ci.get_type() == ChordType::Chord) {
					let li: Line = ci.as_trait().line_dir(e.up());
					if e.up() {
						if (li < l1) && (li < le) { return true; }
					} else {
						if (li > l1) && (li > le) { return true; }
					}
					same_line = li == l4;
				}

				if same_line && (l1 == l4 || le == l4) && cl[1].get_type() == ChordType::Chord {
					if e.up() {
						if (l1 == l4) && (l1 < le) { return true; }
						if (le == l4) && (le < l1) { return true; }
					} else {
						if (l1 == l4) && (l1 > le) { return true; }
						if (le == l4) && (le > l1) { return true; }
					}
				}
			}

			return l1 == le;
		})
	}

	/// adjust stem len for notes between start-end
	/// return 1/4 spatium units
	pub fn adjust(spatiumq: f32, slant: i32, cl: &Vec<ChordRef>) -> i32 {
		let c1 = cl.first().unwrap();
		let c2 = cl.last().unwrap();

		let p1: Point2F = c1.as_trait().stem_pos_beam();
		let p2: Point2F = c2.as_trait().stem_pos_beam();
		let slope = (slant as f32 * spatiumq) / (p2.x - p1.x);
		let mut ml = -1000;
		if c1.as_trait().up() {
			for ci in cl.iter().skip(1) {
				let pi: Point2F = ci.as_trait().stem_pos_beam();
				let y_up = p1.y + (pi.x - p1.x) * slope;
				let l = ((y_up - pi.y) / spatiumq).round() as i32;
				ml = ml.max(l);
			}
		} else {
			for ci in cl.iter().skip(1) {
				let pi: Point2F = ci.as_trait().stem_pos_beam();
				let y_up = p1.y + (pi.x - p1.x) * slope;
				let l = ((pi.y - y_up) / spatiumq).round() as i32;
				ml = ml.max(l);
			}
		}
		ml.max(0)
	}

	/// adjust stem position for single beams
	pub fn adjust_bm(bm: &mut BeamMetric, c1: &ChordRef) {
		const DD: [[i8; 4]; 4] = [
			// St   H  --   S
			[0, 0, 1, 0],  // St
			[0, 0, -1, 0], // S
			[1, 1, 1, -1], // --
			[0, 0, -1, 0], // H
		];
		let ys = (bm.l + c1.as_trait().line().value_i8()) as i32;
		let e1 = ((ys + 1000) % 4).abs();
		let e2 = ((ys + 1000 + bm.s as i32) % 4).abs();
		bm.l -= DD[e1 as usize][e2 as usize]
	}

	pub fn slant_table(interval: usize) -> &'static [i8; 5] {
		const T: [[i8; 5]; 8] = [
			[0, -1, 0, 0, 0],
			[1, -1, 0, 0, 0],
			[3, 4, 2, -1, 0],
			[4, 5, -1, 0, 0],
			[5, -1, 0, 0, 0],
			[5, 6, -1, 0, 0],
			[6, 5, 7, -1, 0],
			[6, 7, 5, 8, -1],
		];
		&T[interval.min(7)]
	}

	/// Kind of verified:
	/// TODO: edge cases
	pub fn compute_stem_len(er: El<Beam>, cl: &Vec<ChordRef>, py1: &mut f32, beam_levels: i32) {
		if cl.is_empty() { return; }

		let spatium = er.borrow_el().spatium();
		let spatiumq = spatium / 4.;
		let zero_slant = Self::zero_slant(er.clone(), cl);

		let c1 = cl.first().unwrap();
		let c2 = cl.last().unwrap();
		let dx: f32 = c2.as_trait().page_pos().x - c1.as_trait().page_pos().x;

		let l1l: Line = c1.as_trait().line();
		let l2l: Line = c2.as_trait().line();
		let l1 = l1l.value_i32() * 2;
		let l2 = l2l.value_i32() * 2;

		// shorten stem length if grace notes beam is under main notes beam.
		// Value 4 estimated. Desired: to find a good formula.
		let grace_steml_corr = if er.borrow_el().is_grace() {
			if let ChordRef::Chord(c) = c1 { // TODO: try into?
				if c.borrow_el().under_beam() { 4 } else { 3 }
			} else { 0 }
		} else { 0 };

		let mut bm = er.with(|e| match beam_levels {
			1 => {
				let mut bm = BeamMetric::get(e.up(), l1 as i8 / 2, l2 as i8 / 2);
				if e.has_no_slope() { bm.s = 0; }

				// special case for two beamed notes: flatten to max of 1sp
				const MAX_SHORT_SLANT: i8 = 4;
				if bm.l != 0 && e.element_count() == 2 {
					if bm.s > MAX_SHORT_SLANT {
						// slant downward; lengthen first stem if down
						if bm.l > 0 { bm.l += bm.s - MAX_SHORT_SLANT }
						bm.s = MAX_SHORT_SLANT; // flatten beam
					} else if bm.s < -MAX_SHORT_SLANT {
						// slant upward; lengthen first stem if up
						if bm.l > 0 { bm.l -= -MAX_SHORT_SLANT - bm.s }
						bm.s = -MAX_SHORT_SLANT; // flatten beam
					}
				}

				if bm.l != 0 {
					if bm.l > 0 { bm.l -= grace_steml_corr } else { bm.l += grace_steml_corr }
				}

				if bm.l != 0 && !(zero_slant && cl.len() > 2) {
					if cl.len() > 2 {
						if e.up() {
							bm.l = (-12 - Self::adjust(spatiumq, bm.s as i32, cl)) as i8;
						} else {
							bm.l = (12 + Self::adjust(spatiumq, bm.s as i32, cl)) as i8;
						}
						Self::adjust_bm(&mut bm, c1);
					}
				} else {
					let mut st: &[i8] = Self::slant_table(if zero_slant { 0 } else { ((l2 - l1) / 2).abs() as usize });
					let mut ll1; // correct checkpint
					// TODO: double l1?
					if e.up() {
						ll1 = l1 - (if (l1 & 3) > 0 { 11 } else { 12 });
						let ll1m = l1 - 10;
						let mut rll1 = ll1;
						if (l1 > 20) && (l2 > 20) {
							st = Self::slant_table(if zero_slant { 0 } else { 1 });
							rll1 = if zero_slant || (l2 < l1) { 9 } else { 8 }
						}

						let mut n = 0;
						loop {
							ll1 -= 1;
							for st in st.iter().take_while(|v| **v != -1) {
								let slant = if l2 > l1 { *st } else { -st };
								let lll1 = rll1.min(ll1m - n - Self::adjust(spatiumq, slant as i32, cl));
								let ll2 = lll1 + slant as i32;
								const BA: [[bool; 4]; 4] = [
									[true, true, false, true],
									[true, true, false, true],
									[false, false, false, true],
									[true, true, false, true]
								];
								if BA[(lll1 & 3) as usize][(ll2 & 3) as usize] {
									ll1 = lll1;
									bm.s = slant;
									break;
								}
							}
							if st[0] != -1 { break }
							n += 1;
							if n > 4 {
								warn!("Beam note not found 1");
								break;
							}
						}
					} else {
						ll1 = l1 + (if (l1 & 3) > 0 { 11 } else { 12 });
						let mut rll1 = ll1;
						if (l1 < -4) && (l2 < -4) {
							st = Self::slant_table(if zero_slant { 0 } else { 1 });
							rll1 = if zero_slant || (l2 > l1) { 7 } else { 8 }
						}
						let mut n = 0;
						loop {
							ll1 += 1;
							let mut i = 0;
							for st in st.iter().take_while(|v| **v != -1) {
								i += 1;
								let slant = if l2 > l1 { *st } else { -st };
								let lll1 = rll1.min(ll1 + Self::adjust(spatiumq, slant as i32, cl));
								let e1 = lll1 & 3;
								let ll2 = lll1 + slant as i32;
								let e2 = ll2 & 2;
								const BA: [[bool; 4]; 4] = [
									[true, true, false, true],
									[true, true, false, true],
									[false, false, false, true],
									[true, true, false, true]
								];
								if BA[e1 as usize][e2 as usize] {
									ll1 = lll1;
									bm.s = slant;
									break;
								}
							}
							if st[i] != -1 { break }
							n += 1;
							if n > 4 {
								warn!("Beam not found 2");
								break;
							}
						}
					}
					bm.l = (ll1 - l1) as i8;
				}
				return bm;
			},
			_ => unimplemented!()
		});

		if er.borrow_el().is_grace() && beam_levels > 1 && bm.l != 0 {
			if bm.l > 0 { bm.l -= grace_steml_corr } else { bm.l += grace_steml_corr }
		}

		er.borrow_mut_el().set_slope({
			if dx == 0.0 { 0.0 } else { (bm.s as f32 * spatiumq) / dx }
		});
		let up: bool = er.borrow_el().up();
		let dy = (c1.as_trait().line_dir(up) - c1.as_trait().line_dir(!up)).value().points(spatium);
		let mut first_stem_len_points = bm.l as f32 * spatiumq;
		let sgn = if first_stem_len_points < 0. { -1.0 } else { 1.0 };
		let p1: Point2F = c1.as_trait().stem_pos_beam();

		for cr in cl.iter().cloned().filter_map(|e| TryInto::<El<Chord>>::try_into(e).ok()) {
			cr.with(|cr| {
				let min_abs_len = cr.min_abs_stem_length();
				let p2 = cr.stem_pos_beam();
				let cr_stem_abs_len = ((p2.x - p1.x) * er.borrow_el().slope() - p2.y + p1.y + first_stem_len_points).abs();
				if cr_stem_abs_len < min_abs_len {
					let dl = min_abs_len - cr_stem_abs_len;
					first_stem_len_points += sgn * dl;
					bm.l += (sgn * dl / spatiumq).round() as i8;
				}
			});
		}

		*py1 += (dy + bm.l as f32) * spatiumq;
		let i = 0;
	}

	pub fn layout_fragment(er: El<Beam>, crl: &Vec<ChordRef>, sv: SpannerSegmentVariant, fragment_idx: usize) {
		if crl.is_empty() { return; }

		if er.borrow_el().distribute() { /* TODO: e.borrow_el().score().respace(crl); */ }
		let c1 = crl.first().unwrap();
		let c2 = crl.last().unwrap();

		let beam_levels: i32 = crl.iter().map(|c|
			c.as_trait().duration_type().hook_count()
		).max().unwrap_or(1).min(1);

		let d_idx = match er.borrow_el().beam_direction() {
			DirectionV::Auto | DirectionV::Down => 0,
			DirectionV::Up => 1,
		};
		let mut f = er.borrow_el().fragments()[fragment_idx].clone();
		let mut py: &mut Point2F = &mut f.py[d_idx];
		let n = crl.len();

		if er.borrow_el().cross() {} else {
			py.x = c1.with(|c| c.stem_pos().y);
			py.y = c2.with(|c| c.stem_pos().y);
			// TODO: this probably gives an error mut
			Self::compute_stem_len(er.clone(), crl, &mut py.x, beam_levels);
			// TODO: compute stem length
		}

		let beam_min_len = er.borrow_el().style().value_p(StyleName::BeamMinLen) * er.borrow_el().scale();
		let mut beam_dist = 0.;

		let page_pos = er.borrow_el().page_pos();

		er.with(|er| {
			let correction = if beam_levels == 4 { 4. / 3. } else { 1. };
			beam_dist = er.style().value_p(StyleName::BeamWidth)
				* (1. + er.style().value_f32(StyleName::BeamDistance) * correction)
				* er.scale() * c1.as_trait().staff().with_d(|s| s.mag(&c1.as_trait().time()), 1.);

			let px1 = c1.with(|c| c.stem_pos_x() + c.page_pos_x());
			let px2 = c2.with(|c| c.stem_pos_x() + c.page_pos_x());

			py.y = (px2 - px1) * er.slope() + py.x;
			py.y -= page_pos.y;
			py.x -= page_pos.y;
		});

		// tesyed till here

		// Create beam segments
		let mut x1 = crl[0].as_trait().stem_pos_x() * crl[0].as_trait().page_pos_x() - er.borrow_el().page_pos_x();
		let mut base_level = 0; // beam level that covers all notes of beam
		let mut cr_base = vec![0; crl.len()]; // offset of beam level 0 for each chord
		let grow_down = er.borrow_el().up();
		let spatium = er.borrow_el().spatium();

		let mut beam_segments = Vec::new();

		for beam_level in 0..beam_levels {
			// loop through the different groups for this beam level
			// inner loop will advance through chordrests within each group
			let mut i = 0;
			while i < n {
				// &ChordRef
				let cr1 = crl[i].clone();
				let duration_type: DurationType = cr1.as_trait().duration_type().ty();
				let l1 = duration_type.hook_type().count() - 1;

				if (cr1.get_type() == ChordType::Rest && i > 0) || l1 < beam_level {
					i += 1;
					continue;
				}

				// at the beginning of a group
				// loop through chordrests looking for end
				let current_cr_index = i;
				i += 1;
				let mut b32 = false;
				let mut b64 = false;
				while i < n {
					// &ChordRef
					let c = crl[i].clone();
					let p = if i != 0 { Some(crl[i - 1].clone()) } else { None };
					let l = c.as_trait().duration_type().hook_count() - 1;

					let bm = Groups::end_beam(&c, &p);
					b32 = (beam_level >= 1) && (bm == BeamMode::Begin32);
					b64 = (beam_level >= 2) && (bm == BeamMode::Begin64);

					if (l >= beam_level && (b32 || b64)) || (l < beam_level) {
						if i > 1 && crl[i - 1].get_type() == ChordType::Rest {
							i -= 1;
						}
						break;
					}

					i += 1;
				}
				// Seems correct till here

				// found end of group
				let chord_rest_end_group_index = i;
				let cr2 = crl[chord_rest_end_group_index - 1].clone();

				// if group covers whole beam, we are still at base level
				if current_cr_index == 0 && chord_rest_end_group_index == n {
					base_level = beam_level;
				}

				// default assumption - everything grows in same direction
				let mut bl = if grow_down { beam_level } else { -beam_level };
				let mut grow_down_group = grow_down;

				// calculate direction for this group
				let cr1_up = cr1.as_trait().up();
				let cr2_up = cr2.as_trait().up();
				if beam_level > base_level {
					if (current_cr_index > 0 && cr1_up == cr2_up)
						|| (current_cr_index == n && cr1_up != cr2_up) {
						// matching direction for outer stems, not first group
						// or, opposing direction for outer stems, last group
						// recalculate beam for this group based on its *first* cr
						grow_down_group = cr1_up;
					} else if current_cr_index != 0 && chord_rest_end_group_index < n && cr1_up != cr2_up {
						// opposing directions for outer stems, first (but not only) group
						// recalculate beam for this group if necessary based on its *last* cr
						grow_down_group = cr2_up;
					}

					// recalculate segment offset bl
					let base = cr_base[current_cr_index];
					if grow_down_group && base <= 0 {
						bl = base + beam_level;
					} else if grow_down_group {
						bl = base + 1;
					} else if !grow_down_group && base >= 0 {
						bl = base - beam_level;
					} else if !grow_down_group {
						bl = base - 1;
					}
				}

				// if there are more beam levels,
				// record current beam offsets for all notes of this group for re-use
				if beam_level < beam_levels - 1 {
					for i1 in current_cr_index..chord_rest_end_group_index {
						cr_base[i1] = bl;
					}
				}


				let stem_width = if let ChordRef::Chord(cr1) = cr1.clone() {
					cr1.borrow_el().stem().unwrap().borrow_el().line_width()
				} else {
					0.
				};
				let mut x2 = cr1.as_trait().stem_pos_x() + cr1.as_trait().page_pos_x() - page_pos.x;
				let mut x3;

				if (chord_rest_end_group_index - current_cr_index) > 1 {
					// TODO: poor assumptuin that crl is sorted?
					let chord_rest_2 = crl[chord_rest_end_group_index - 1].clone();
					x3 = chord_rest_2.as_trait().stem_pos_x() + chord_rest_2.as_trait().page_pos_x() - page_pos.x;
					// TODO: stem pos x and stempos.x differ return values which is inconsistent af

					if cr1_up {
						x2 -= stem_width;
					} else if !chord_rest_2.as_trait().up() {
						x3 += 1.; // TODO: If chord_rest_2 is chord. Add line width
					}
				} else {
					// create broken segment
					match cr1 {
						ChordRef::Rest(_) => { continue },
						ChordRef::Chord(cr1) => {
							let size_chord_rests = crl.len();
							let mut len = beam_min_len as f32;

							//
							// find direction (by default, segment points to right)
							//
							// if first or last of group (including tuplet groups)
							// unconditionally set beam at right or left side
							let tuplet = cr1.borrow_el().tuplet();
							if current_cr_index == 0 {} else if current_cr_index == size_chord_rests - 1 {
								len = -len;
							} else if tuplet.is_some() && Some(DurationElementRef::Chord(cr1.clone())) == tuplet.with_d(|tuplet| tuplet.first_element(), None) {} else if tuplet.is_some() && Some(DurationElementRef::Chord(cr1.clone())) == tuplet.with_d(|tuplet| tuplet.last_element(), None) {
								len = -len;
							} else if b32 || b64 {
								len -= len;
							} else if !cr1.borrow_el().is_grace() {
								/* inside group - here it gets more complex
								see http://musescore.org/en/node/42856, http://musescore.org/en/node/40806
								our strategy:
								decide if we have reached the end of a "logical" grouping
								even if we are not literally at the end of a beam group
								we do this two ways:
								1) see if beam groups would have indicated a break or sub-beam if the next chord were same length as this
								2) see if next note is on a "sub-beat" as defined by 2 * current note duration
								in either case, broken segment should point left; otherwise right
								however, we should try to be careful to avoid "floating" segments
								caused by mismatches between number of incoming versus outgoing beams
								so, we favor the side with more beams (to the extent we can count reliably)
								if there is a corner case missed, this would probably be where */

								let prev_cr = crl[current_cr_index - 1].clone();
								let next_cr = crl[current_cr_index + 1].clone();
								let current_duration = cr1.borrow_el().duration_type().clone();
								let current_hooks = current_duration.hook_count();

								// since we have already established that we are not at end of sub-beam,
								// outgoing beams should always be # hooks of next chord
								let beams_out = next_cr.as_trait().duration_type().hook_count();

								// incoming beams is normally # hooks of previous chord
								// unless this is start of sub-beam
								// TODO: assuming we are attached to a staff
								let measure_tick = cr1.borrow_el().measure().with_d(|m| m.time(), Fraction::default());
								let g = cr1.borrow_el().staff().with(|s| s.group(&measure_tick)).unwrap();
								let stretch = cr1.borrow_el().staff().with(|s| s.timestretch(&measure_tick)).unwrap();
								let current_tick = (cr1.borrow_el().rel_time() * stretch).ticks();
								let mut bm = g.beam_mode(current_tick, current_duration.ty());
								let beams_in = match bm {
									BeamMode::Begin32 => 1,
									BeamMode::Begin64 => 2,
									_ => prev_cr.as_trait().duration_type().hook_count(),
								};

								// remember, we are checking whether nextCR would have started sub-beam *if* same duration as this
								let next_tick: i32 = (next_cr.as_trait().rel_time() * stretch).ticks();
								bm = g.beam_mode(next_tick, current_duration.ty());

								if current_hooks - beams_out > 1 && beams_in > beams_out && current_hooks > beams_in {
									// point left to avoid floating segment
									len = -len;
								} else if beams_in < beams_out {
									// point right to avoid floating segment
								} else if bm != BeamMode::Auto {
									// beam group info suggests this is a logical group end as per 1) above
									len = -len;
								} else {
									// determine if this is a logical group end as per 2) above
									let base_tick = tuplet.with_d(|t| t.time(), measure_tick);
									let mut tick_next = next_cr.as_trait().time() - base_tick;
									tuplet.with(|tuplet| {
										// for tuplets with odd ratios, apply ratio
										// thus, we are performing calculation relative to apparent rather than actual beat
										// for tuplets with even ratios, use actual beat
										// see https://musescore.org/en/node/58061

										let r = tuplet.ratio().clone();
										if r.numerator & 1 > 0 {
											tick_next = tick_next * r;
										}
									});

									// determine the tick length of a chord with one beam level less than this
									// (i.e. twice the ticks of this)
									let tick_mod = measure_tick.ticks() * 2;

									// if this completes, within the measure or tuplet, a unit of tickMod length, flip beam to left
									// (allow some tolerance for tick rounding in tuplets
									// without tuplet tolerance, could be simplified)
									const BEAM_TUPLET_TOLERANCE: i32 = 6;
									let mmod = tick_next.ticks() % tick_mod;
									if mmod <= BEAM_TUPLET_TOLERANCE || (tick_mod - mmod) <= BEAM_TUPLET_TOLERANCE {
										len = -len;
									}
								}
							}

							let stem_up = cr1_up;
							if stem_up && len > 0. {
								x1 -= stem_width;
							} else if !stem_up && len < 0. {
								x2 += stem_width;
							}
							x3 = x2 + len;
						},
					}
				}

				let yo = py.x + bl as f32 * beam_dist * er.borrow_el().grow_left();
				let yoo = py.x + bl as f32 * beam_dist * er.borrow_el().grow_right();
				let ly1 = (x2 - x1) * er.borrow_el().slope() + yo;
				let ly2 = (x3 - x1) * er.borrow_el().slope() + yoo;

				// TODO: check for infinite / anan values
				beam_segments.push(LineF::new(Point2F::new(x2, ly1), Point2F::new(x3, ly2)))
			}
		}

		er.with_mut(|mut er| {
			er.set_segments(beam_segments.clone());
		});


		//  calculate stem length
		let slope = er.borrow_el().slope();
		for cr in crl {
			let mut index = 0;
			match cr {
				ChordRef::Rest(_) => {},
				ChordRef::Chord(c) => c.with(|c| {
					if c.hook().is_some() {
						// TODO: remove hook
					}
					index += 1;

					let stem_pos = Point2F::new(c.stem_pos_x() + c.page_pos_x(), c.page_pos().y); // TODO: correct x by page pos
					let x2 = stem_pos.x - page_pos.x; // Make relative to beam
					let y1 = (x2 - x1) * slope + py.x + page_pos.y; // Add page pos (
					let y2 = stem_pos.y;
					let fuzz = spatium * 4.; // something is wrong

					let mut by = if y2 < y1 { -1000000. } else { 1000000. };
					for l in er.borrow_el().beam_segments() {
						if (x2 + fuzz) >= l.x1() && (x2 - fuzz) <= l.x2() {
							let y = (x2 - l.x2()) * slope + l.y1();
							by = if y2 < y1 { by.max(y) } else { by.min(y) }
						} else {
							let u = 0;
						}
					}

					if by == -1000000. || by == 1000000. {
						if beam_segments.is_empty() {
							println!("No Beam segemnts");
						} else {
							println!("BeamSegment not found: x {}  {:?}-{:?}",
							         x2,
							         beam_segments.first().map(|s| s.x1()),
							         beam_segments.last().map(|s| s.x2())
							);
						}
						by = 0.;
					}

					c.stem().with_mut(|mut stem| {
						let mut sw2 = stem.line_width() / 2.;
						if c.up() { sw2 = -sw2; }
						stem.set_pos(Point2F::new(c.stem_pos_x() + sw2, c.stem_pos().y));
						let l = y2 - (by + page_pos.y);
						stem.set_len(Spatium(l / spatium));

						// let stem_slash = c.stem_slash();
						// TODO: telayout stem slash
						// TODO: relayout tremeolo
					});

					if let Some(stem) = c.stem().cloned() {
						StemRenderer::layout(stem);
					}
				}),
			}
		}
		// TODO: test
		// TODO: seems to calculate oke now. But need to handle the beam rendering itself. and file the stems
	}

	/// TODO: revise
	pub fn create_beams(e: El<Measure>) {
		let score = e.borrow_el().score().clone();
		let cross_measure = score.style().value_bool(StyleName::CrossMeasureValues);
		let measure_time = e.borrow_el().time();
		for track in 0..score.track_count() as Track {
			let staff = match score.staff(track2staff(track as Track)) {
				Some(staff) => staff,
				None => continue,
			};

			// Don’t compute beams for invisible staffs
			if staff.with(|staff| !staff.show()) {
				continue
			}

			let ts = staff.borrow_el().timesig(&measure_time).cloned();
			let stretch = ts.with_d(|ts| ts.stretch().clone(), Fraction::new(1, 1));
			let mut beat_subdividion: HashMap<i32, Duration> = HashMap::new();
			let mut check_beats = false;
			let mut first_cr = true;

			// If this measure is simple meter (actually X/4),
			// then perform a prepass to determine the subdivision of each beat
			if ts.with_d(|ts| ts.denominator() == 4, false) {
				check_beats = true;
				for (_, segment) in e.borrow_el().segments().iter_ty(Fraction::zero(), SegmentTypeMask::CHORDREST) {
					if let Some(cr) = segment.borrow_el().chordrest(track) {
						let beat = (segment.borrow_el().rel_time() * stretch).ticks() / constants::DIVISION;
						let duration: Duration = cr.as_trait().duration_type().clone();
						if beat_subdividion.contains_key(&beat) {
							beat_subdividion.insert(beat, duration.min(beat_subdividion[&beat].clone()));
						} else {
							beat_subdividion.insert(beat, duration);
						}
					}
				}
			}

			let mut start_cr: Option<ChordRef> = None;
			let mut prev_cr: Option<ChordRef> = None;
			let mut beam: Option<El<Beam>> = None;

			for cr in e.borrow_el().segments().iter_vals()
				.filter_map(|s: _| s.borrow_el().chordrest(track)) {
				if first_cr {
					first_cr = false;
					// TODO: move into a function or reduce nesting
					// Handle cross-measure beams
					let mode: BeamMode = cr.as_trait().beam_mode();
					if mode == BeamMode::Mid || mode == BeamMode::End {
						let prev_cr = score.find_chordrest(measure_time - Fraction::from_ticks(1), track);
						if let Some(prev_cr_t) = prev_cr {
							let prev_m: Option<El<Measure>> = prev_cr_t.as_trait().measure();
							if let Some(prev_m) = prev_m {
								let duration_type: DurationType = prev_cr_t.as_trait().duration_type().ty();

								if !prev_cr_t.as_trait().beam_mode().no_continue()
									&& prev_m.with(|m| !m.line_break() && !m.page_break() && !m.section_break())
									&& duration_type >= DurationType::Eighth
									&& duration_type <= DurationType::D1024th
								{
									beam = prev_cr_t.as_trait().beam();
									// when beam is found, a1 is no longer required.
									start_cr = if beam.is_some() { None } else { Some(prev_cr_t) };
								}
							}
						}
					}
				}

				// Handle grace notes and cross-measure beaming
				if let ChordRef::Chord(c) = &cr {
					// TODO: beam grace notes before
					// TODO: beam grace notes after
					// set up for cross-measure values as soon as possible
					// to have all computations (stems, hooks, ...) consistent with it
					if !c.borrow_el().is_grace() {
						// TODO: cross measure setup
					}
				}

				let mut bm = if let ChordRef::Rest(r) = &cr {
					BeamMode::None // do not beam rests set to Beam::Mode::AUTO
				} else {
					Groups::end_beam(&cr, &prev_cr) // get defaults from time signature properties
				};

				// perform additional context-dependent checks
				if bm == BeamMode::Auto {
					// check if we need to break beams according to minimum duration in current / previous beat
					if check_beats && !cr.as_trait().rel_time().is_zero() {
						let time: Fraction = cr.as_trait().rel_time() * stretch;
						// check if on the beat
						if (time.ticks() % constants::DIVISION) == 0 {
							let beat = time.ticks() / constants::DIVISION;
							// get minimum duration for this & previous beat
							let min_duration = beat_subdividion[&beat].clone()
								.min(beat_subdividion[&(beat - 1)].clone());
							// re-calculate beam as if this were the duration of current chordrest
							cr.as_trait_mut().set_duration_type(min_duration);
							let (save_duration, save_cm_duration, save_cm_val) = cr.with(|cr: Ref<dyn ChordRestTrait>| {
								(cr.duration_type().clone(), cr.cm_duration_type().clone(), cr.cross_measure())
							});
							bm = Groups::end_beam(&cr, &prev_cr);
							cr.with_mut(|mut cr: RefMut<dyn ChordRestTrait>| {
								cr.set_duration_type(save_duration.clone());
								cr.set_cm_duration_type(save_cm_duration.clone());
								cr.set_cross_measure(save_cm_val);
							})
						}
					}
				}
				prev_cr = Some(cr.clone());

				// if chord has hooks and is 2nd element of a cross-measure value
				// set beam mode to NONE (do not combine with following chord beam/hook, if any)
				if cr.with(|cr| cr.duration_type().hook_count() > 0 && cr.cross_measure() == CrossMeasure::Second) {
					bm = BeamMode::None;
				}

				if cr.with(|cr| cr.duration_type().ty() <= DurationType::Quarter || bm == BeamMode::None) {
					let mut remove_beam = true;
					if let Some(b) = beam {
						// TODO: layout beam
						remove_beam = b.borrow_el().element_count() <= 1;
						beam = None;
					}
					if let Some(a) = start_cr {
						if remove_beam {
							// TODO: remove from beam. delete beam if empty
							a.as_trait_mut().remove_beam(false);
						}
						start_cr = None;
					}

					if let Some(start_cr) = &start_cr {
						start_cr.as_trait_mut().remove_beam(false);
					}
				}

				let mut next_c = false;
				if let Some(b) = &beam {
					if bm != BeamMode::Begin {
						cr.as_trait_mut().replace_beam(b.clone());
						next_c = true;
					}
					if bm == BeamMode::Begin || bm == BeamMode::End {
						// TODO: beam layout
						beam = None;
					}
				}
				if next_c { continue }

				if let Some(start_cr_ref) = &start_cr {
					if !bm.is_mid() && (bm == BeamMode::Begin
						|| start_cr_ref.as_trait().segment_type() != cr.as_trait().segment_type()
						|| start_cr_ref.as_trait().time() + start_cr_ref.as_trait().actual_duration() < cr.as_trait().time()
					) {
						start_cr_ref.as_trait_mut().remove_beam(true);
						start_cr = Some(cr);
					} else {
						let mut beam = start_cr_ref.as_trait().beam();
						if beam.is_none() || beam.with_d(|b| b.elements().next() != start_cr.as_ref(), false) {
							beam = Some(Beam::new(e.borrow_el().score().clone()).with_mut_i(|mut beam| {
								beam.set_generated(true);
								beam.set_track(track);
							}));
							start_cr_ref.as_trait_mut().replace_beam(beam.clone().expect("Not sure if it is conditionally impossible"));
						}
						cr.as_trait_mut().replace_beam(beam.expect("Not sure if it is conditionally impossible"));
						start_cr = None;
					}
				} else {
					start_cr = Some(cr);
				}
			}

			if let Some(beam) = &beam {
				// TODO: layout
			} else if let Some(start_cr) = start_cr {
				let next_tick = start_cr.with(|c| c.time() + c.actual_duration());
				let m = if next_tick >= e.borrow_el().end_time() {
					//TODO: e.borrow_el().next_measure()
					unimplemented!()
				} else {
					e.clone()
				};
				let next_cr = m.with(|m| m.find_chordrest(next_tick, track));
				let b: Option<El<Beam>> = start_cr.as_trait().beam();
				if b.with_d(|b| b.elements().next() == Some(&start_cr) && next_cr.is_some() && next_cr.as_ref().unwrap().as_trait().beam_mode().is_mid(), false) {
					start_cr.as_trait_mut().remove_beam(false);
				}
			}
		}
	}
}