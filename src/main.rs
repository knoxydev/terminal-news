#![allow(warnings)]

// MODULES
mod set_key;
pub use crate::set_key::api_fn;

mod news_mod;
pub use crate::news_mod::news_fn;

mod settings;
pub use crate::settings::stn_fn;


fn main() {
	println!("Enter 'api_key', 'settings' or 'news' -> ");

	let mut resp_old = String::new();
	std::io::stdin().read_line(&mut resp_old).expect("Failes");
	let resp = &resp_old[0..&resp_old.len() - 2];

	if resp == "api_key" { api_fn::set_api_key(); }
	else if resp == "news" { news_fn::show_news(); }
	else if resp == "settings" {stn_fn::start(); }
	else {
		println!("Incorrect command !");
		return;
	}
}
