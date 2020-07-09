use std::fs;
use hyper;
use tokio;

mod bencoding;
mod torrent;

use torrent::torrent::Torrent;
use torrent::tracker_info::TrackerInfo;

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
    let tracker_info = TrackerInfo::from(response_data)?;
    println!("tracker_info: {}", tracker_info);
    
    Ok(())
}
