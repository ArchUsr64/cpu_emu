use json::JsonValue;

extern crate json;

pub fn parse_config() -> JsonValue {
	let filename = "config.json";
	let f = std::fs::read_to_string(filename);
	let f = match f {
		Ok(file) => file,
		Err(_) => {
			println!("'{}' not found", filename);
			std::process::exit(0);
		}
	};
	let config = json::parse(f.as_str()).expect("Failed to parse '{}' successfully");
	config
}
