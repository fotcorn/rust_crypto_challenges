extern crate rustc_serialize;
use rustc_serialize::hex::ToHex;

fn main() {
	let string = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes();
	let key = "ICE".as_bytes();

	let mut encrypted = Vec::new();

	for i in 0..string.len() {
		encrypted.push(string[i] ^ key[i % 3])
	}

	println!("{}", encrypted.to_hex());
}
