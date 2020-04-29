use std::{str, fs};
use std::net::Ipv4Addr;
use sha1::Digest;
use hyper;
use tokio;
use percent_encoding;
use byteorder::{ByteOrder, BigEndian};

mod bencoding;
mod torrent;

use bencoding::bencode::Bencode;
use bencoding::byte_string::ByteString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let filename = "tmp/raspbian-buster-lite.zip.torrent";
    let input = fs::read(filename).expect("Unable to read file");

    let data = bencoding::decoder::decode(input);

    println!("{}", data);

    let mut torrent = match data {
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
    let sha1_encoded = percent_encoding::percent_encode(sha1.as_slice(), percent_encoding::NON_ALPHANUMERIC).to_string();

    let announce_str = match torrent.remove(&ByteString::from_str("announce")) {
        Some(info) => info,
        None => panic!("\"announce\" key did not exist in torrent file."),
    };

    let mut announce_vec = match announce_str {
        Bencode::ByteString(s) => s,
        _ => panic!("\"announce\" key was not a string."),
    };

    for &byte in b"?info_hash=" {
        announce_vec.push(byte);
    }

    for &byte in sha1_encoded.as_bytes() {
        announce_vec.push(byte);
    }

    let announce_url = match str::from_utf8(announce_vec.as_slice()) {
        Ok(v) => v,
        Err(_) => panic!("alert"),
    };
    
    let uri: hyper::Uri = announce_url.parse().unwrap();

    let client = hyper::Client::new();

    let resp = client.get(uri).await?;
    
    // And then, if the request gets a response...
    println!("status: {}", resp.status());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(resp).await?;
    let vec = buf.to_vec();

    let mut tracker_info = match bencoding::decoder::decode(vec) {
        Bencode::Dict(dict) => dict,
        _ => panic!("Could not decode the torrent file."),
    };

    let peers = match tracker_info.remove(&ByteString::from_str("peers")) {
        Some(info) => info,
        None => panic!("\"info\" key did not exist in torrent file."),
    };

    let peers_array = match peers {
        Bencode::ByteString(s) => s,
        _ => panic!("\"peers\" wasn't a ByteString."),
    };

    let mut ip_addrs = Vec::<String>::new();

    for peer in peers_array.chunks(6) {
        let addr = Ipv4Addr::from(BigEndian::read_u32(&peer[..4]));
        let port = BigEndian::read_u16(&peer[4..]);
        ip_addrs.push(format!("{}:{}", addr, port));
    }

    println!("peers_array: {:?}", ip_addrs);
    
    Ok(())
}
