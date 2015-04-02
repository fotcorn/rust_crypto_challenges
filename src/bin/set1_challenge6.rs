extern crate rustc_serialize;
use rustc_serialize::base64::FromBase64;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;

fn main() {
	let data = match load_from_file() {
		Err(why) => panic!("Failed to read data/set1_challenge4.txt: {}", why),
		Ok(data) => data,
	};

	let key_size = guess_key_size(&data);
	println!("guesses key size: {}", key_size);

	let blocks = split_into_blocks(&data, key_size);

	println!("Blocks count (should be key_size): {}", blocks.len());

	let mut key: Vec<u8> = vec!(0; key_size);
	for i in 0..blocks.len() {
		key[i] = bruteforce_xor(&blocks[i]);
	}

	let mut decrypted: Vec<u8> = Vec::new();

	for i in 0..data.len() {
		decrypted.push(data[i] ^ key[i % key_size])
	}

	let string = str::from_utf8(&decrypted).unwrap();
	println!("{}", string);

}

fn load_from_file() -> std::io::Result<Vec<u8>> {
	let file = try!(File::open("data/set1_challenge6.txt"));
	let reader = BufReader::new(file);

	let mut data = String::new();

	for line in reader.lines() {
		data.push_str(&(line.unwrap()));
	}

	let decoded_data = data.from_base64().unwrap();
	return Ok(decoded_data);
}

fn guess_key_size(data: &[u8]) -> usize {
	let mut smallest_key_size_distance = std::usize::MAX;
	let mut guessed_key_size = 0;

	for key_size in 2..41 {
		let str1 = &data[0..key_size]; // slice?
		let str2 = &data[key_size..key_size*2];
		let str3 = &data[key_size*2..key_size*3];
		let str4 = &data[key_size*3..key_size*4];

		let mut distances = Vec::new();

		distances.push(hamming_distance(str1, str2) / key_size)                                                                          ;
		distances.push(hamming_distance(str1, str3) / key_size);
		distances.push(hamming_distance(str1, str4) / key_size);
		distances.push(hamming_distance(str2, str3) / key_size);
		distances.push(hamming_distance(str2, str4) / key_size);
		distances.push(hamming_distance(str3, str4) / key_size)                                                                          ;

		let average_distance = distances.iter().fold(0, |total, &value| total + value);

		if average_distance < smallest_key_size_distance {
			smallest_key_size_distance = average_distance;
			guessed_key_size = key_size;
		}
	}
	return guessed_key_size;
}

/*
binary hamming distance
this is a test
wokka wokka!!!
-> 37
 */
fn hamming_distance(str1: &[u8], str2: &[u8]) -> usize {
    let mut distance = 0;

    for i in 0..str1.len() {
		let diff = str1[i] ^ str2[i];
        if (diff & 0b00000001) > 0 { distance += 1; }
        if (diff & 0b00000010) > 0 { distance += 1; }
        if (diff & 0b00000100) > 0 { distance += 1; }
        if (diff & 0b00001000) > 0 { distance += 1; }
        if (diff & 0b00010000) > 0 { distance += 1; }
        if (diff & 0b00100000) > 0 { distance += 1; }
        if (diff & 0b01000000) > 0 { distance += 1; }
        if (diff & 0b10000000) > 0 { distance += 1; }
    }
    return distance;
}


fn split_into_blocks(data: &[u8], key_size: usize) -> Vec<Vec<u8>> {
	let mut blocks: Vec<Vec<u8>> = vec![Vec::new(); key_size];
	for i in 0..data.len() {
		blocks[i % key_size].push(data[i]);
	}
	return blocks;
}


fn bruteforce_xor(a: &Vec<u8>) -> u8 {
	let mut vec: Vec<u8> = vec![0; a.len()];

	let mut max_score: u64 = 0;
	let mut best_char: u8 = 0;

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
			best_char = char;
		}
	}
	return best_char;
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