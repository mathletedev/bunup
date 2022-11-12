use std::{env::current_dir, fs::read_to_string, process::Command};

use serde_json::{from_str, Value};

fn parse_deps(v: Value) -> Vec<String> {
	v.as_object()
		.unwrap()
		.keys()
		.map(|e| e.to_string())
		.collect::<Vec<String>>()
}

fn main() {
	let data: Value = from_str(
		&read_to_string(current_dir().unwrap().to_str().unwrap().to_string() + "/package.json")
			.expect("Failed to read package.json"),
	)
	.expect("Failed to parse package.json");

	Command::new("bun")
		.arg("add")
		.args(parse_deps(data["dependencies"].clone()))
		.spawn()
		.expect("Failed to execute bun")
		.wait()
		.unwrap();

	Command::new("bun")
		.arg("add")
		.arg("-d")
		.args(parse_deps(data["devDependencies"].clone()))
		.spawn()
		.expect("Failed to execute bun")
		.wait()
		.unwrap();
}
