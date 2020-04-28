use std::fs;
use sha1::Digest;

mod bencoding;

use bencoding::bencode::Bencode;
use bencoding::byte_string::ByteString;

fn main() {
    let filename = "tmp/raspbian-buster-lite.zip.torrent";
    let input = fs::read(filename).expect("Unable to read file");

    let mut torrent = match bencoding::decoder::decode(input) {
        Bencode::Dict(dict) => dict,
        _ => panic!("Could not decode the torrent file."),
    };

    let torrent_info = match torrent.remove(&ByteString::from_str("info")) {
        Some(info) => info,
        None => panic!("\"info\" key did not exist in torrent file."),
    };

    let mut hasher = sha1::Sha1::new();
    let encoded_info = bencoding::encoder::encode(torrent_info);
    hasher.input(encoded_info);
    let sha1 = hasher.result();

    println!("{:?}", sha1);
}
