use super::*;
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct Sym {
	pub(super) code: i32,
	pub(super) index: u32,
	pub(super) bbox: RectF,
	pub(super) advance: f32,
	pub(super) stem_down_nw: Point2F,
	pub(super) stem_up_se: Point2F,
	pub(super) cut_out_ne: Point2F,
	pub(super) cut_out_nw: Point2F,
	pub(super) cut_out_se: Point2F,
	pub(super) cut_out_sw: Point2F,
	pub(super) compound_ids: Vec<SymId>
}

impl Default for Sym {
	fn default() -> Self { Self {
		code: -1,
		index: 0,
		bbox: RectF::default(),
		advance: 0.,
		stem_down_nw: Point2F::default(),
		stem_up_se: Point2F::default(),
		cut_out_ne: Point2F::default(),
		cut_out_nw: Point2F::default(),
		cut_out_se: Point2F::default(),
		cut_out_sw: Point2F::default(),
		compound_ids: Vec::new(),
	}}
}

impl Sym {
	pub fn is_valid(&self) -> bool { self.code != -1 }

	pub const fn code(&self) -> i32 { self.code }

	pub fn get_char(&self) -> Option<char> {
		std::char::from_u32(self.code.try_into().ok()?)
	}

	pub const fn index(&self) -> u32 { self.index }

	pub const fn bbox(&self) -> &RectF { &self.bbox }

	pub const fn advance(&self) -> f32 { self.advance }

	pub fn stem_down_nw(&self) -> Point2F { self.stem_down_nw - self.bbox.origin.to_vector() }

	pub fn stem_up_se(&self) -> Point2F { self.stem_up_se - self.bbox.origin.to_vector() }

	pub const fn cut_out_ne(&self) -> &Point2F { &self.cut_out_ne }

	pub const fn cut_out_nw(&self) -> &Point2F { &self.cut_out_nw }

	pub const fn cut_out_se(&self) -> &Point2F { &self.cut_out_se }

	pub const fn cut_out_sw(&self) -> &Point2F { &self.cut_out_sw }

	pub const fn compound_ids(&self) -> &Vec<SymId> { &self.compound_ids }
}