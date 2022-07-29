pub mod news_fn {
	#![allow(warnings)]

	// PACKAGES
	use tokio::io;
	use std::fs;
	use std::fs::File;

	use std::result::Result;
	use std::error::Error;
	use std::path::Path;

	use reqwest::{Client, header::USER_AGENT};
	use serde::Deserialize;
	use serde_json::Value;

	// MODULES
	pub use crate::set_key::api_fn;


	pub fn show_news() {
		let fl_path = String::from("./target/api_key.txt");

		if Path::new(&fl_path).exists() == true {
			let data = fs::read_to_string(&fl_path).expect("Something went wrong reading the file");
			do_request(&data);
		} else {
			println!("You didn't insert API key. You need API key to read the news !");
			return api_fn::set_api_key();
		}
	}

	#[tokio::main]
	pub async fn do_request(url: &String) -> Result<(), Box<dyn Error>> {
		let new_url = format!("https://newsapi.org/v2/top-headlines?country=ru&apiKey={}", url);

		let client = Client::new();
		let resp = client.get(&new_url)
			.header(USER_AGENT, "reqwest")
			.send()
			.await?
			.text()
			.await?;

		let mut file = File::create("./target/data.json").expect("Error encountered while creating file!");
		fs::write("./target/data.json", &resp).expect("Unable to write file");
		

		let v: Value = serde_json::from_str(&resp)?;
		let sub_value: Vec<Value> = serde_json::from_str(&v["articles"].to_string())?;

		for i in &sub_value {
			println!("{:?}\n", i["title"]);
		}
		
		Ok(())
	}
}