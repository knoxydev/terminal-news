pub mod api_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::path::Path;
	use std::fs;
	use std::fs::File;
	use std::io::prelude::*;
	use std::result::Result;
	use std::error::Error;

	fn create_folder() -> Result<(), Box<dyn Error>> {
		fs::create_dir_all("./settings-fld")?;
		Ok(())
	}

	pub fn set_api_key() {
		create_folder();
		let fl_path = String::from("./settings-fld/api_key.txt");

		println!("You can take your API key from -> https://newsapi.org/\n");
		println!("Enter 'api_key' -> ");

		let mut resp_old = String::new();
		std::io::stdin().read_line(&mut resp_old).expect("Failes");
		let resp = &resp_old[0..&resp_old.len() - 2];

		if resp == "" { return println!("empty !\n'api_key' do not received !"); }

		if Path::new(&fl_path).exists() == true
		{
			fs::write(&fl_path, resp).expect("Unable to write file");
		}
		else
		{
			let mut file = File::create(&fl_path).expect("Error encountered while creating file!");
			fs::write(&fl_path, resp).expect("Unable to write file");
		}

		println!("done !");
	}
}