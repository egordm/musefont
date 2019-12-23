use std::{env, path::Path, time::Instant};
use rust_swig::{PythonConfig, LanguageConfig};
use std::{fs::File, io::{Write, Read}};

const MODULE_NAME: &str = "musefont_python";

fn main() {
	let now = Instant::now();
	let out_dir = env::var("OUT_DIR").unwrap();

	let inputs = vec![
		//Path::new("src/glue_enums.rs.in"),
		Path::new("src/glue_utils.rs.in"),
		Path::new("src/glue_lib.rs.in"),
	];
	let interm = &Path::new(&out_dir).join("glue.rs.in");
	let output = &Path::new(&out_dir).join("glue.rs");

	merge_files(&inputs, interm);
	rust_swig_expand(interm, output);

	println!("rust swig expand times: {}", now.elapsed().as_secs_f32());

	for input in inputs {
		println!("cargo:rerun-if-changed=src/{}", input.to_str().unwrap());
	}
	println!("cargo:rerun-if-changed=src/lib.rs");
}

fn rust_swig_expand(from: &Path, out: &Path) {
	println!("Run {}", MODULE_NAME);
	let python_cfg = PythonConfig::new(MODULE_NAME.to_owned());
	let swig_gen = rust_swig::Generator::new(LanguageConfig::PythonConfig(python_cfg));
	swig_gen.expand(MODULE_NAME, from, out);
}

fn merge_files(inputs: &[&Path], output: &Path) {
	let mut output = File::create(output).unwrap();

	for input in inputs {
		let mut input = File::open(input).unwrap();
		let mut input_data = String::new();
		input.read_to_string(&mut input_data).unwrap();
		write!(output, "{}", input_data).unwrap();
	}
}