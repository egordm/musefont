#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum DirectionH {
	Left = 0,
	Right = 1,
	Auto = 2,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum DirectionV {
	Down = 0,
	Up = 1,
	Auto = 2,
}