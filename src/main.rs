use std::fs;
use std::io::Read;
use tokio;
mod bencoding;
mod torrent;
mod client;
use torrent::torrent::Torrent;
use client::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let filename = "tmp/raspbian-buster-lite.zip.torrent";
    let file = fs::File::open(filename)?;
    let input = fs::read(filename).expect("Unable to read file");

    let stream_data = bencoding::stream_decoder::decode(&mut file.bytes())?;

    let data = bencoding::decoder::decode(input);
    assert_eq!(data, stream_data);
    let torrent = Torrent::from(stream_data)?;

    let mut client = Client::new(torrent);
    let tracker_info = client.tracker_info().await?;

    println!("tracker_info: {}", tracker_info);
    
    Ok(())
}
