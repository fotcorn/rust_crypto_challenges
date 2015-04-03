use std::str;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

extern crate rustc_serialize;
use rustc_serialize::base64::FromBase64;

extern crate crypto;
use crypto::{aes, blockmodes, buffer};
use crypto::symmetriccipher::{Decryptor};
use crypto::buffer::{ReadBuffer, WriteBuffer};


fn main() {
	let key = "YELLOW SUBMARINE".as_bytes();
	let encrypted_data = load_from_file("data/set1_challenge7.txt").unwrap();

	let decrypted = decrypt(&encrypted_data, key);

	let string = str::from_utf8(&decrypted).unwrap();
	println!("{}", string);
}

fn load_from_file(filename: &str) -> std::io::Result<Vec<u8>> {
	let file = try!(File::open(filename));
	let reader = BufReader::new(file);

	let mut data = String::new();

	for line in reader.lines() {
		data.push_str(&(line.unwrap()));
	}

	let decoded_data = data.from_base64().unwrap();
	Ok(decoded_data)
}

fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut decryptor = aes::ecb_decryptor(
            aes::KeySize::KeySize128,
            key,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            buffer::BufferResult::BufferUnderflow => break,
            buffer::BufferResult::BufferOverflow => { }
        }
    }

    final_result
}
