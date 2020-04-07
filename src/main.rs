use std::fs;

mod bencoding;

fn main() {
    let filename = "tmp/raspbian-buster-lite.zip.torrent";
    let result = fs::read(filename).expect("Unable to read file");

    let input: &[u8] = result.as_ref();
    let result = bencoding::decoder::decode(input);
    println!("{}", result);
}
