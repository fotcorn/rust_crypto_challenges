use std::str::{from_utf8};

extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex};

fn main() {
	let a = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();


	let mut vec: Vec<u8> = Vec::new();
	for _ in 0..a.len() {
		vec.push(0);
	}

	'char_loop: for char in 0..255 {
		for i in 0..a.len() {
			match a[i] ^ char {
				0x20...0x7E => {
					vec[i] = a[i] ^ char;
				}
				_ => {
					continue 'char_loop;
				}
			}
		}
		match from_utf8(&vec) {
			Ok(v) => {
				println!("{}", v);
			}
            _ => {}
		}
	}
}
