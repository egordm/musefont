use crate::*;
use log::debug;
use std::borrow::Borrow;

// TODO: implement after staff and measure are implemented.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpannerSegmentType {
	SINGLE,
	BEGIN,
	MIDDLE,
	END
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BeamMode {
	Auto,
	Begin,
	Mid,
	End,
	None,
	Begin32,
	Begin64,
	Invalid
}

#[derive(Clone, Debug, Default)]
struct BeamFragment {
	py: [Point2F; 2],
}

#[derive(Clone, Debug)]
pub struct Beam {
	element: Element,
	chords: Vec<Elem<Chord>>,
	segments: Vec<LineF>,
	fragments: Vec<BeamFragment>,
	direction: DirectionV,

	up: bool,
	distribute: bool,
	no_slope: bool,
	is_grace: bool,
	cross: bool,
	grow_left: f32,
	grow_right: f32,
	beam_dist: f32,

	min_move: i32,
	max_move: i32,
	max_duration: Duration,
	slope: f32,
}

impl Beam {
	pub fn new(score: Score) -> Elem<Self> {
		let mut ret = Elem::new(Self {
			element: Element::new(score),
			chords: vec![],
			segments: vec![],
			fragments: vec![],
			direction: DirectionV::Auto,
			up: true,
			distribute: false,
			no_slope: false,
			is_grace: false,
			cross: false,
			grow_left: 1.0,
			grow_right: 1.0,
			beam_dist: 0.0,
			min_move: 0,
			max_move: 0,
			max_duration: Default::default(),
			slope: 0.0
		});
		ret.apply_style();
		ret
	}

	pub fn up(&self) -> bool { self.up }
	pub fn set_up(&mut self, v: bool) { self.up = v }
	pub fn no_slope(&self) -> bool { self.no_slope }
	pub fn set_no_slope(&mut self, v: bool) { self.no_slope = v }
	pub fn beam_direction(&self) -> DirectionV { self.direction }
	pub fn set_beam_direction(&mut self, v: DirectionV) { self.direction = v }

	pub fn grow_left(&self) -> f32 { self.grow_left }
	pub fn set_grow_left(&mut self, v: f32) { self.grow_left = v }
	pub fn grow_right(&self) -> f32 { self.grow_right }
	pub fn set_grow_right(&mut self, v: f32) { self.grow_right = v }
	pub fn distribute(&self) -> bool { self.distribute }
	pub fn set_distribute(&mut self, v: bool) { self.distribute = v }

	pub fn contains(&self, e: &Elem<Chord>) -> bool { self.chords.contains(&e) }
	pub fn empty(&self) -> bool { self.chords.is_empty() }
	pub fn add_chord(&mut self, e: Elem<Chord>) {
		e.borrow_mut().set_beam(Some(Chord::transform_ref(self.self_ref()).unwrap().downgrade()));
		if !self.chords.contains(&e) {
			// insert element in same order as it appears in the score
			self.chords.push(e);
		}
	}
	pub fn remove_chord(&mut self, e: &Elem<Chord>) {
		if let Some(p) = self.chords.iter().position(|x| x == e) {
			self.chords.remove(p);
		} else {
			debug!("Beam::remove_chord(): cannot find Chord");
		}
		e.borrow_mut().set_beam(None);
	}

	pub fn beam_pos(&self) -> Point2F {
		if let Some(f) = self.fragments.last() {
			let idx = if self.direction == DirectionV::Auto || self.direction == DirectionV::Down { 0 } else { 1 };
			f.py[idx] / self.spatium()
		} else {
			Point2F::default()
		}
	}
	pub fn set_beam_pos(&mut self, bp: Point2F) {
		if self.fragments.is_empty() { self.fragments.push(BeamFragment::default()) }
		let spatium = self.spatium();
		let idx = if self.direction == DirectionV::Auto || self.direction == DirectionV::Down { 0 } else { 1 };
		let f = self.fragments.last_mut().unwrap();
		// TODO: user modified idx
		f.py[idx] = bp * spatium;
	}

	fn layout2(&mut self, crl: &[Elem<Chord>], segment_type: SpannerSegmentType, fragment: i32) {
		//if self.distribute { self.score().respace(crl) } TODO: respace

		if let (Some(c1), Some(c2)) = (crl.first(), crl.last()) {
			let mut beam_levels = 1;
			for c in crl {
				beam_levels = beam_levels.max(c.borrow().duration().hook_type().count());
			}

			let didx = if self.direction == DirectionV::Auto || self.direction == DirectionV::Down { 0 } else { 1 };
			let py = self.fragments[fragment as usize].py[didx];

			let spatium = self.spatium();
			let beam_min_len = self.score().style().value_p(StyleId::BeamMinLen as SId) * self.scale();

			let beam_width_st = self.score().style().value_p(StyleId::BeamWidth as SId);
			let beam_dist_st = self.score().style().value_f32(StyleId::BeamDistance as SId);
			if beam_levels == 4 {
				self.beam_dist = beam_width_st * (1. + beam_dist_st * 4. / 3.);
			} else {
				self.beam_dist = beam_width_st * (1. + beam_dist_st);
			}
			self.beam_dist *= self.scale();
			// TODO: beam dist *- staff tick scale

			let base_level = 0;
			let n = crl.len();
			let cr_base = vec![0; n];
			let grow_down = self.up;

			for beam_level in 0..beam_levels {
				// loop through the different groups for this beam level
				// inner loop will advance through chordrests within each group
				for i in 0..n {
					let cr1 = &crl[i];
					let l1 = cr1.borrow().duration().hook_count() - 1;

					// TODO: if rest continue
					if l1 < beam_level { continue; }

					// at the beginning of a group loop through chordrests looking for end
					let current_chord_rest_index = i;
					let b32 = false;
					let b64 = false;
					for _ in i + 1..n {
						let c = &crl[i];
						let p = unimplemented!();
						let l = c.borrow().duration().hook_count() - 1;
						// TODO: continue
					}
				}
			}

		}
	}
}

fn end_beam(cr: &Elem<Chord>, prev: &Elem<Chord>) -> BeamMode {
	if cr.borrow().is_grace() || cr.borrow().beam_mode() != BeamMode::Auto {
		cr.borrow().beam_mode()
	} else {
		let d = cr.borrow().duration().ty();
		unimplemented!()
	}
}

impl Drawable for Beam {
	fn layout(&mut self) {
		self.set_bbox(RectF::default());
		let st = SpannerSegmentType::SINGLE;
		if self.fragments.len() < 1 {
			self.fragments.push(BeamFragment::default());
		}
		// layout2

		let spatium = self.spatium();
		let lw2 = self.score().style().value_spatium(StyleId::BeamWidth as SId) * spatium * 0.5 * self.scale();

		for bs in self.segments.iter().cloned() {
			let xl = bs.x1().min(bs.x2());
			let xr = bs.x1().max(bs.x2());
			let yd = bs.y1().min(bs.y2());
			let yu = bs.y1().max(bs.y2());
			let r = rect_adjust(RectF::new(
				Point2F::new(xl, yd),
				Size2F::new(xr - xl, yu - yd)
			), Point2F::new(0.0, -lw2), Point2F::new(0.0, lw2));
			self.element.add_bbox(&r);
		}

	}

	fn draw(&self, painter: PainterRef) {
		unimplemented!()
	}
}

impl ElementTrait for Beam {
	fn el(&self) -> &Element {&self.element.el() }
	fn el_mut(&mut self) -> &mut Element { self.element.el_mut() }
	fn element_type(&self) -> ElementType { ElementType::Hook }

	fn apply_style(&mut self) {
		self.no_slope = self.score().style().value_bool(StyleId::BeamNoSlope as SId);
	}
}
