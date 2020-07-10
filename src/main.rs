use std::fs;
use tokio;

mod bencoding;
mod torrent;
mod client;

use torrent::torrent::Torrent;
use client::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let filename = "tmp/raspbian-buster-lite.zip.torrent";
    let input = fs::read(filename).expect("Unable to read file");

    let data = bencoding::decoder::decode(input);
    let torrent = Torrent::from(data)?;

    let client = Client::new(torrent);
    let tracker_info = client.tracker_info().await?;

    println!("tracker_info: {}", tracker_info);
    
    Ok(())
}
