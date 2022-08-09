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

	use dialoguer::{Select, theme::ColorfulTheme};
	use console::Term;

	#[derive(Deserialize, Serialize, Debug)]
	struct Structura {
		category: Vec<String>,
		lang: Vec<String>
	}

	#[derive(Deserialize, Serialize, Debug)]
	struct Settings {
		ctg: String,
		lng: String,
		zen: String
	}


	fn create_folder() -> Result<(), Box<dyn Error>> {
		fs::create_dir_all("./settings-fld")?;
		Ok(())
	}

	fn save(s_ctg: String, s_lng: String, s_zen: String) {
		let obj = Settings { ctg: s_ctg.to_string(), lng: s_lng.to_string(), zen: s_zen.to_string() };
		fs::write("./settings-fld/settings.json", serde_json::to_string_pretty(&obj).unwrap()).expect("Unable to write file");
	}

	fn first_request(data: &Vec<String>, stn: &Settings) -> Result<String, Box<dyn Error>> {
		clearscreen::clear().unwrap();

		println!("Now - {:?}\n", stn.ctg);
		println!("Category ->");

		let mut ctg = String::new();
		let items = &data;
		let selection = Select::with_theme(&ColorfulTheme::default())
			.items(&items)
			.default(0)
			.interact_on_opt(&Term::stderr())?;

		match selection {
			Some(index) => {
				ctg = items[index].to_string();
				println!("User selected item : {}", items[index]);
			},
			None => println!("User did not select anything")
		}

		Ok(ctg)
	}

	fn second_request(data: &Vec<String>, stn: &Settings) -> Result<String, Box<dyn Error>> {
		clearscreen::clear().unwrap();

		println!("Now - {:?}\n", stn.lng);
		println!("Country ->");

		let mut lng = String::new();
		let items = &data;
		let selection = Select::with_theme(&ColorfulTheme::default())
			.items(&items)
			.default(0)
			.interact_on_opt(&Term::stderr())?;

		match selection {
			Some(index) => {
				lng = items[index].to_string();
				println!("User selected item : {}", items[index]);
			},
			None => println!("User did not select anything")
		}

		Ok(lng)
	}

	fn third_request(stn: &Settings) -> Result<String, Box<dyn Error>> {
		clearscreen::clear().unwrap();

		println!("Now - {:?}\n", stn.zen);
		println!("Zen Mode:");

		let mut zen = String::new();
		let items = vec!["ON", "OFF"];
		let selection = Select::with_theme(&ColorfulTheme::default())
			.items(&items)
			.default(0)
			.interact_on_opt(&Term::stderr())?;

		match selection {
			Some(index) => {
				zen = items[index].to_string();
				println!("User selected item : {}", items[index]);
			},
			None => println!("User did not select anything")
		}

		Ok(zen)
	}
	
	pub fn start() {
		create_folder();
		let fl_path = String::from("./settings-fld/settings.json");

		if Path::new(&fl_path).exists() == false {
			File::create(&fl_path).expect("Error encountered while creating file!");

			let obj = Settings { ctg: "all".to_string(), lng: "us".to_string(), zen: "off".to_string() };
			fs::write(&fl_path, serde_json::to_string_pretty(&obj).unwrap()).expect("Unable to write file");
		}

		let res = Structura {
      category: vec!["all".into(),"business".into(),"entertainment".into(),"general".into(),"health".into(),"science".into(),"sports".into(),"technology".into()],
      lang: vec!["ae".into(),"au".into(),"be".into(),"bg".into(),"br".into(),"ch".into(),"cn".into(),"cz".into(),"fr".into(),"gb".into(),"gr".into(),"it".into(),"jp".into(),"kr".into(),"lt".into(),"lv".into(),"ma".into(),"ng".into(),"nl".into(),"ph".into(),"ro".into(),"ru".into(),"sg".into(),"si".into(),"sk".into(),"tw".into(),"ua".into(),"us".into()],
    };

		let data_two = fs::read_to_string(&fl_path).expect("wrong 1");
		let mut stn: Settings = serde_json::from_str(&data_two.to_string()).expect("wrong 2");

		let ctg = first_request(&res.category, &stn).unwrap().to_lowercase();
		let lng = second_request(&res.lang, &stn).unwrap().to_lowercase();
		let zen = third_request(&stn).unwrap().to_lowercase();

		save(ctg, lng, zen);
	}
}