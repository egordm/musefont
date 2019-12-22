pub struct Font {

}

pub struct Size {

}

pub struct Symbol {
	code: i32,
	index: u32,
}

pub type SymbolId = u32;

pub struct HelloWorld {

}

impl HelloWorld {
	pub fn new() -> Self {
		Self {}
	}

	pub fn hello_world(&self) -> String {
		"ehllo world".to_string()
	}
}