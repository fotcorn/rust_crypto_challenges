use std::str;

extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex};

fn main() {
	let a = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();


	let mut vec: Vec<u8> = Vec::new();
	for _ in 0..a.len() {
		vec.push(0);
	}

	let mut max_score: u64 = 0;
	let mut best_string: Vec<u8> = Vec::new();

	'char_loop: for char in 0..255 {
		let mut score: u64 = 0;
		for i in 0..a.len() {
			match a[i] ^ char {
				0x20...0x7E => {
					vec[i] = a[i] ^ char;
					score += get_score(vec[i] as char);
				}
				_ => {
					continue 'char_loop;
				}
			}
		}
		if score > max_score {
			max_score = score;
			best_string = vec.clone();
		}
	}
	println!("{}: {}", max_score, str::from_utf8(&best_string).unwrap());
}

fn get_score(character: char) -> u64 {
	return match character.to_uppercase().next().unwrap() {
		'E' => 4452,
		'T' => 3305,
		'A' => 2865,
		'O' => 2723,
		'I' => 2697,
		'N' => 2578,
		'S' => 2321,
		'R' => 2238,
		'H' => 1801,
		'L' => 1450,
		'D' => 1360,
		'C' => 1192,
		'U' => 973,
		'M' => 895,
		'F' => 856,
		'P' => 761,
		'G' => 666,
		'W' => 597,
		'Y' => 593,
		'B' => 529,
		'V' => 375,
		'K' => 193,
		'X' => 84,
		'J' => 57,
		'Q' => 43,
		'Z' => 32,
		_ => 0,
	}
}
