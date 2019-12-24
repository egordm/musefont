#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum DirectionH {
	Auto = 0,
	Left = 1,
	Right = 2
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum DirectionV {
	Auto = 0,
	Up = 1,
	Down = 2
}