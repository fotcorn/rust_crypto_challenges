use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

extern crate rustc_serialize;
use rustc_serialize::hex::FromHex;

fn main() {
	let lines = load_from_file();

	let mut max_duplicates = 0;
	let mut line_with_max_duplicates = String::new();

	for line in lines {
		let decoded = line.from_hex().unwrap();

		let duplicates = find_duplicates_substrings(decoded);
		if duplicates > max_duplicates {
			max_duplicates = duplicates;
			line_with_max_duplicates = line;
		}
	}

	println!("Max duplicates: {}", max_duplicates);
	println!("{}", line_with_max_duplicates);
}

fn load_from_file() -> Vec<String> {
	let file = File::open("data/set1_challenge8.txt").unwrap();
	let reader = BufReader::new(file);

	let mut lines = Vec::<String>::new();

	for line in reader.lines() {
		lines.push(line.unwrap());
	}
	lines
}

fn find_duplicates_substrings(string: Vec<u8>) -> u32 {
	let mut substrings = HashMap::<Vec<u8>, u32>::new();
	let block_count = string.len() / 16;

	for i in 0..block_count {
		let slice = &string[i*16..(i+1)*16];
		let count = match substrings.get(slice) {
			Some(c) =>  c + 1,
			None => 0,
		};
		let mut v = Vec::new();
		v.extend(slice.iter().map(|&i| i));
		substrings.insert(v, count);
	}

	substrings.values().fold(0, |total, &value| total + value)
}
