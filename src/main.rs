#![allow(warnings)]

// PACKAGES


// MODULES
mod set_key;
pub use crate::set_key::api_fn;


fn main() {
	println!("Enter 'api_key' or 'news' -> ");

	let mut resp_old = String::new();
	std::io::stdin().read_line(&mut resp_old).expect("Failes");
	let resp = &resp_old[0..&resp_old.len() - 2];

	if resp == "api_key" { api_fn::set_api_key(); }
	else if resp == "news" {
		println!("HELLO");
	} else {
		println!("Incorrect command !");
		return;
	}
}
