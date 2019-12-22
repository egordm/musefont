use std::{env, path::Path, time::Instant};
use rust_swig::{PythonConfig, LanguageConfig};

const MODULE_NAME: &str = "musefont_python";

fn main() {
	let now = Instant::now();
	let out_dir = env::var("OUT_DIR").unwrap();

	rust_swig_expand(
		Path::new("src/glue.rs.in"),
		&Path::new(&out_dir).join("glue.rs"),
	);

	let expand_time = now.elapsed();
	println!(
		"rust swig expand times: {}",
		expand_time.as_secs() as f64 + (expand_time.subsec_nanos() as f64) / 1_000_000_000.
	);
	println!("cargo:rerun-if-changed=src/glue.rs.in");
	println!("cargo:rerun-if-changed=src/lib.rs");
}

fn rust_swig_expand(from: &Path, out: &Path) {
	println!("Run {}", MODULE_NAME);
	let python_cfg = PythonConfig::new(MODULE_NAME.to_owned());
	let swig_gen = rust_swig::Generator::new(LanguageConfig::PythonConfig(python_cfg));
	swig_gen.expand(MODULE_NAME, from, out);
}