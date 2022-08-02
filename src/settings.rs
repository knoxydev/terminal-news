pub mod stn_fn {
	// PACKAGE
	use serde::Deserialize;
	use serde_json::Value;
	use colored::Colorize;
	use std::path::Path;
	use std::fs::File;
	
	pub fn start() {
		print!("\x1B[2J\x1B[1;1H");
		//println!("{}", "this is red".red());

		let json_file_path = Path::new("./parameters/settings.json");
		let file = File::open(json_file_path).expect("file not found");
		let sub_value: Vec<Value> = serde_json::from_str(&v["settings"].to_string())?;

		for i in &sub_value {
			println!("{:?}\n", i);
		}
	}
}