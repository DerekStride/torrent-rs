use std::fmt;
use std::result::Result;

use crate::torrent::torrent::Torrent;
use crate::torrent::tracker_info::TrackerInfo;
use crate::bencoding::decoder;
use crate::client::error::Error;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Client {
    torrent: Torrent,
    tracker_info: Option<TrackerInfo>,
}

impl Client {
    pub fn new(torrent: Torrent) -> Self {
        Self {
            torrent: torrent,
            tracker_info: None,
        }
    }

    pub async fn tracker_info(&self) -> Result<TrackerInfo, Error> {
        let announce_url = self.torrent.announce_url()?;
        let uri: hyper::Uri = announce_url.parse()?;
        let client = hyper::Client::new();

        let resp = client.get(uri).await?;
        let buf = hyper::body::to_bytes(resp).await?;
        let response_data = decoder::decode(buf.to_vec());

        Ok(TrackerInfo::from(response_data)?)
    }
}

impl fmt::Display for Client {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Client {{ torrent: \"{}\" }}", self.torrent.info.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    use crate::bencoding::decoder::decode;
    use crate::torrent::torrent::Torrent;
    use crate::torrent::torrent_info::TorrentInfo;

    fn client() -> Client {
        let torrent = Torrent {
            announce: mockito::server_url(),
            created_by: "derekstride".to_string(),
            encoding: "UTF-8".to_string(),
            creation_date: 170,
            info: TorrentInfo {
                length: 4,
                name: "derek.jar".to_string(),
                piece_length: 100,
                private: true,
                pieces: vec![b'z', 195, 40],
            },
        };
        Client::new(torrent)
    }

    fn tracker_info_struct() -> TrackerInfo {
        let bencode = decode(tracker_info_str().as_bytes().to_vec());
        match TrackerInfo::from(bencode) {
            Ok(t) => t,
            Err(_) => panic!("Can not decode tracker_info_str()"),
        }
    }

    fn tracker_info_str() -> &'static str {
        "d8:completei4e10:downloadedi6e10:incompletei1e8:intervali1906e12:min intervali953e5:peers6:Oz\x00x\x1AAe"
    }

    #[tokio::test]
    async fn test_mock() {
        let expected = tracker_info_struct();
        let client = client();
        let m = mock("GET", Matcher::Regex(".*".into()))
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(tracker_info_str())
            .expect_at_least(1)
            .create();

        let tracker_info = match client.tracker_info().await {
            Ok(t) => t,
            Err(e) => panic!(format!("{}", e)),
        };

        m.assert();
        assert_eq!(expected, tracker_info);
    }
}
