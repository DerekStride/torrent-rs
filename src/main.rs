use std::fs;
use std::net::Ipv4Addr;
use hyper;
use tokio;
use byteorder::{ByteOrder, BigEndian};

mod bencoding;
mod torrent;

use bencoding::bencode::Bencode;
use bencoding::byte_string::ByteString;
use torrent::torrent::Torrent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let filename = "tmp/raspbian-buster-lite.zip.torrent";
    let input = fs::read(filename).expect("Unable to read file");

    let data = bencoding::decoder::decode(input);
    let torrent = Torrent::from(data)?;

    let announce_url = torrent.announce_url()?;
    
    let uri: hyper::Uri = announce_url.parse().unwrap();

    let client = hyper::Client::new();

    let resp = client.get(uri).await?;
    
    // And then, if the request gets a response...
    println!("status: {}", resp.status());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(resp).await?;
    let vec = buf.to_vec();

    let response_data = bencoding::decoder::decode(vec);
    println!("tracker_info: {}", response_data);

    let mut tracker_info = match response_data {
        Bencode::Dict(dict) => dict,
        _ => panic!("Could not decode the torrent file."),
    };

    let peers = match tracker_info.remove(&ByteString::from_str("peers")) {
        Some(info) => info,
        None => panic!("\"peers\" key did not exist in torrent file."),
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
