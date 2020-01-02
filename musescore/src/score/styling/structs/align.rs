
#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Placement {
	Above = 0,
	Below = 1,
}

bitflags! { pub struct Align: u32 {
	const LEFT     = 0;
	const RIGHT    = 1;
	const HCENTER  = 2;
	const TOP      = 0;
	const BOTTOM   = 4;
	const VCENTER  = 8;
	const BASELINE = 16;
	const CENTER = Self::HCENTER.bits | Self::VCENTER.bits;
	const HMASK  = Self::LEFT.bits    | Self::RIGHT.bits    | Self::HCENTER.bits;
	const VMASK  = Self::TOP.bits     | Self::BOTTOM.bits   | Self::VCENTER.bits | Self::BASELINE.bits;
}}

impl Into<u32> for Align {
	fn into(self) -> u32 { self.bits }
}