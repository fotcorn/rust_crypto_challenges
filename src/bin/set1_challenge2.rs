extern crate rustc_serialize;

use rustc_serialize::hex::{FromHex, ToHex};

fn main() {
	let a = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
	let b = "686974207468652062756c6c277320657965".from_hex().unwrap();

	let mut vec = Vec::new();

	for i in 0..a.len() {
		vec.push(a[i] ^ b[i]);
	}
	println!("{}", vec.to_hex());
}
