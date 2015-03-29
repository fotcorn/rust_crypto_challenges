extern crate rustc_serialize;

use rustc_serialize::hex::FromHex;
use rustc_serialize::base64::{ToBase64, STANDARD};

fn main() {
	let x = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".from_hex().unwrap();
	let b = x.to_base64(STANDARD);
	println!("{}", b);
}
