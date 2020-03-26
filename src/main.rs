use std::fs;
use std::str;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod bencoding;

fn main() {
    let filename = "tmp/raspbian-buster-lite.zip.torrent";
    let file = File::open(filename).unwrap();

    // Read and print file
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = [0u8; 32];
    // let result = buf_reader.read(&mut contents);

    // println!("{:?}", result);
    // println!("{:?}", contents);

    // Process file as bytes
    // let result = match fs::read(filename) {
    //     val => val,
    //     Err(e) => println!("{}", e),
    // }
    let result = fs::read(filename).expect("Unable to read file");
    // println!("{:?}", str::from_utf8(&mut c));

    let input: &[u8] = result.as_ref();
    let result = bencoding::decoder::decode(input);
    println!("{:?}", result);
}
