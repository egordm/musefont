use crate::score::*;
use crate::drawing::{PainterRef};
use std::collections::HashMap;
use crate::constants;
use crate::score::StyleName::BeamMinLen;
use crate::score::PropertyId::P1;
use bitflags::_core::option::Option::Some;

pub struct BeamRenderer {}

impl Renderer<Beam> for BeamRenderer {
	fn layout(_e: El<Beam>) {
		unimplemented!()
	}

	fn render(_e: El<Beam>, _state: &mut RendererState, _painter: PainterRef) {
		unimplemented!()
	}
}

impl BeamRenderer {
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
