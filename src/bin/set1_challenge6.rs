extern crate rustc_serialize;
use rustc_serialize::base64::FromBase64;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
	let data = match load_from_file() {
		Err(why) => panic!("Failed to read data/set1_challenge4.txt: {}", why),
		Ok(data) => data,
	};

	let mut smallest_keysize_distance = std::usize::MAX;
	let mut smallest_keysize = 0;

	for key_size in 2..41 {
		let str1 = &data[0..key_size]; // slice?
		let str2 = &data[key_size..key_size*2];
		let str3 = &data[key_size*2..key_size*3];
		let str4 = &data[key_size*3..key_size*4];

		let mut distances = Vec::new();

		distances.push(hamming_distance(str1, str2) / key_size);
		distances.push(hamming_distance(str1, str3) / key_size);
		distances.push(hamming_distance(str1, str4) / key_size);
		distances.push(hamming_distance(str2, str3) / key_size);
		distances.push(hamming_distance(str2, str4) / key_size);
		distances.push(hamming_distance(str3, str4) / key_size);

		let average_distance = distances.iter().fold(0, |total, &value| total + value);

		if average_distance < smallest_keysize_distance {
			smallest_keysize_distance = average_distance;
			smallest_keysize = key_size;
		}
	}
	println!("smallest_keysize_distance: {}, key_size: {}", smallest_keysize_distance, smallest_keysize);
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
