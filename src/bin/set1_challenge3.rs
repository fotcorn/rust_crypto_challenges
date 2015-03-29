use std::str::{from_utf8};

extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex, ToHex};

fn main() {
	let a = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();


	let mut vec: Vec<u8> = Vec::new();
	for _ in 0..a.len() {
		vec.push(0);
	}

	for char in 0..255 {
		for i in 0..a.len() {
			vec[i] = a[i] ^ char;
		}
		match from_utf8(&vec) {
			Ok(v) => {
				println!("{}", v);
			}
            Err(e) => {}
		}
	}
}
