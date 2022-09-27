#![allow(warnings)]

// MODULES
mod set_key;
pub use crate::set_key::api_fn;

mod news_mod;
pub use crate::news_mod::news_fn;

mod settings;
pub use crate::settings::stn_fn;


fn input() {
	println!("Enter 'api_key', 'settings', 'news', 'clear', 'about' or 'exit' -> ");

	let mut resp_old = String::new();
	std::io::stdin().read_line(&mut resp_old).expect("Failes");
	let resp = &resp_old[0..&resp_old.len() - 2];

	match resp.as_ref() {
		"api_key" => api_fn::set_api_key(),
		"news" => news_fn::show_news(),
		"settings" => stn_fn::start(),
		"clear" => clearscreen::clear().unwrap(),
		"about" => {
			println!("\nNews in your terminal !");
			println!("The program is written in Rust");
			println!("Source: github.com/nkr413/terminal-news\n");
		},
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}

	input();
}

fn main() {
	input();
}
