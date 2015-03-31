extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex};

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
	match bruteforce_file("data/set1_challenge4.txt") {
		Err(why) => panic!("Failed to read data/set1_challenge4.txt: {}", why),
		_ => (),
	}
}

fn bruteforce_file(path: &str) -> std::io::Result<()> {
	let file = try!(File::open(path));
	let reader = BufReader::new(file);

	let mut best_string = String::new();
	let mut max_score = 0;

	for line in reader.lines() {
		let hex_code = line.unwrap().from_hex().unwrap();

		match bruteforce_xor(&hex_code) {
			Some((string, score)) => {
				if score > max_score {
					max_score = score;
					best_string = string;
				}
			}
			None => (),
		}
	}
	println!("{}: {}", max_score, best_string);
	return Ok(());
}

fn bruteforce_xor(a: &Vec<u8>) -> Option<(String, u64)> {
	let mut vec: Vec<u8> = vec![0; a.len()];

	let mut max_score: u64 = 0;
	let mut best_string: Vec<u8> = Vec::new();

	'char_loop: for char in 0..255 {
		let mut score: u64 = 0;
		for i in 0..a.len() {
			match a[i] ^ char {
				0x20...0x7E|0xA => {
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

	if max_score > 0 {
		Some((String::from_utf8(best_string).unwrap(), max_score))
	} else {
		None
	}
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
