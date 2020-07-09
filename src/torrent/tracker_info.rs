use std::fmt;
use std::result::Result;
use std::net::Ipv4Addr;
use byteorder::{ByteOrder, BigEndian};

use crate::bencoding::bencode::Bencode;
use crate::torrent::peer::Peer;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TrackerInfo {
    pub complete: i64,
    pub downloaded: i64,
    pub incomplete: i64,
    pub interval: i64,
    pub min_interval: i64,
    pub peers: Vec<u8>,
}



impl TrackerInfo {
    pub fn from(input: Bencode) -> Result<Self, String> {
        let complete = input.get_number("complete")?;
        let downloaded = input.get_number("downloaded")?;
        let incomplete = input.get_number("incomplete")?;
        let interval = input.get_number("interval")?;
        let min_interval = input.get_number("min interval")?;
        let peers = input.remove_bytestring("peers")?;

        Ok(
            Self {
                complete,
                downloaded,
                incomplete,
                interval,
                min_interval,
                peers,
            }
        )
    }

    pub fn peer_addrs(&self) -> Vec<Peer> {
        let mut ip_addrs = Vec::<Peer>::new();

        for peer in self.peers.chunks(6) {
            let ip = Ipv4Addr::from(BigEndian::read_u32(&peer[..4]));
            let port = BigEndian::read_u16(&peer[4..]);
            ip_addrs.push(Peer { ip, port })
        }

        return ip_addrs;
    }
}

impl fmt::Display for TrackerInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        format(fmt, self)
    }
}

fn format(fmt: &mut fmt::Formatter, v: &TrackerInfo) -> fmt::Result {
    write!(fmt, "TrackerInfo: {{ ")?;
    write!(fmt, "complete: {}, ", v.complete)?;
    write!(fmt, "downloaded: {}, ", v.downloaded)?;
    write!(fmt, "incomplete: {}, ", v.incomplete)?;
    write!(fmt, "interval: {}, ", v.interval)?;
    write!(fmt, "min_interval: {}, ", v.min_interval)?;
    write!(fmt, "peers: [")?;
    let addrs = v.peer_addrs();
    let len = addrs.len();
    for (i, peer) in addrs.iter().enumerate() {
        write!(fmt, "{}", peer)?;
        if len != i + 1 {
            write!(fmt, ", ")?;
        }
    }
    write!(fmt, "] }}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencoding::decoder::decode;

    fn tracker_info(data: &[u8]) -> Result<TrackerInfo, String> {
        TrackerInfo::from(
            decode(data.to_vec())
        )
    }

    #[test]
    fn test_err_when_input_is_not_a_dictionary() {
        let result = tracker_info(b"le");
        assert_eq!(Err("Bencode is not a dict.".to_string()), result);
    }

    #[test]
    fn test_err_when_input_is_an_empty_dictionary() {
        let result = tracker_info(b"de");
        assert_eq!(Err("\"complete\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_complete_is_present() {
        let result = tracker_info(b"d8:completei4ee");
        assert_eq!(Err("\"downloaded\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_downloaded_is_present() {
        let result = tracker_info(b"d8:completei4e10:downloadedi6ee");
        assert_eq!(Err("\"incomplete\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_incomplete_is_present() {
        let result = tracker_info(b"d8:completei4e10:downloadedi6e10:incompletei1ee");
        assert_eq!(Err("\"interval\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_interval_is_present() {
        let result = tracker_info(b"d8:completei4e10:downloadedi6e10:incompletei1e8:intervali1906ee");
        assert_eq!(Err("\"min interval\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_min_interval_is_present() {
        let result = tracker_info(b"d8:completei4e10:downloadedi6e10:incompletei1e8:intervali1906e12:min intervali953ee");
        assert_eq!(Err("\"peers\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_ok_when_all_values_are_present() {
        let data = b"d8:completei4e10:downloadedi6e10:incompletei1e8:intervali1906e12:min intervali953e5:peers6:Oz\x00x\x1AAe";
        let result = tracker_info(data);
        let expected = TrackerInfo {
            complete: 4,
            downloaded: 6,
            incomplete: 1,
            interval: 1906,
            min_interval: 953,
            peers: vec![79, 122, 0, 120, 26, 65],
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn test_peer_addrs() {
        let tracker_info = TrackerInfo {
            complete: 4,
            downloaded: 6,
            incomplete: 1,
            interval: 1906,
            min_interval: 953,
            peers: vec![79, 122, 0, 120, 26, 65],
        };

        let expected = vec![Peer { ip: Ipv4Addr::new(79, 122, 0, 120), port: 6721 }];

        assert_eq!(expected, tracker_info.peer_addrs());
    }
}
