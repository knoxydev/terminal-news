pub mod stn_fn {
	#![allow(warnings)]

	// PACKAGE
	use serde::{Deserialize, Serialize};
	use colored::Colorize;
	use std::result::Result;
	use std::error::Error;
	use std::path::Path;
	use std::fs::File;
	use std::fs;

	#[derive(Deserialize, Serialize, Debug)]
	struct Structura {
		category: Vec<String>,
		lang: Vec<String>
	}

	#[derive(Deserialize, Serialize, Debug)]
	struct Settings {
		ctg: String,
		lng: String
	}

	fn save(s_ctg: String, s_lng: String) {
		let obj = Settings { ctg: s_ctg.to_string(), lng: s_lng.to_string() };
 		fs::write("./target/settings.json", serde_json::to_string_pretty(&obj).unwrap()).expect("Unable to write file");
	}

	fn first_request(data: &mut Structura, stn: &Settings) -> String {
		print!("\x1B[2J\x1B[1;1H");
		println!("Now - {:?}\n", stn.ctg);
		println!("Categories - {:?}\n", data.category);
		println!("Select a category from the list or select 'nothing' if you want to receive news from all categories -> ");

		let mut resp_old = String::new();
		std::io::stdin().read_line(&mut resp_old).expect("Failes");
		let resp = &resp_old[0..&resp_old.len() - 2];

		return resp.to_string();
	}

	fn second_request(data: &mut Structura, stn: &Settings) -> String {
		print!("\x1B[2J\x1B[1;1H");
		println!("Now - {:?}\n", stn.lng);
		println!("Countries - {:?}\n", data.lang);
		println!("Select 'country' you want to get news for. You also can select 'nothing' -> ");

		let mut resp_old = String::new();
		std::io::stdin().read_line(&mut resp_old).expect("Failes");
		let resp = &resp_old[0..&resp_old.len() - 2];

		return resp.to_string();
	}
	
	pub fn start() {
		let fl_path = String::from("./target/settings.json");

		let data = fs::read_to_string("./parameters/settings.json").expect("wrong 1");
		let mut res: Structura = serde_json::from_str(&data.to_string()).expect("wrong 2");

		//if resp == "" { return println!("empty !\n'api_key' do not received !"); }

		if Path::new(&fl_path).exists() == false {
 			File::create(&fl_path).expect("Error encountered while creating file!");

 			let obj = Settings { ctg: "nothing".to_string(), lng: "nothing".to_string() };
 			fs::write(&fl_path, serde_json::to_string_pretty(&obj).unwrap()).expect("Unable to write file");
		}

		let data_two = fs::read_to_string(&fl_path).expect("wrong 1");
		let mut stn: Settings = serde_json::from_str(&data_two.to_string()).expect("wrong 2");

		let ctg = first_request(&mut res, &stn);
		let lng = second_request(&mut res, &stn);

		save(ctg, lng);
	}
}