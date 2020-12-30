use std::{str, fmt};
use std::result::Result;

use crate::bencoding::bencode::Bencode;
use crate::torrent::torrent_info::TorrentInfo;
use crate::torrent::error::Error;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Torrent {
    pub announce: String,
    pub created_by: String,
    pub creation_date: i64,
    pub encoding: String,
    pub info: TorrentInfo,
}

impl Torrent {
    pub fn from(input: Bencode) -> Result<Self, Error> {        
        let announce = input.get_string("announce")?;
        let created_by = input.get_string("created by")?;
        let encoding = input.get_string("encoding")?;
        let creation_date = input.get_number("creation date")?;
        let info = TorrentInfo::from(input.remove("info")?)?;
        
        Ok(
            Self {
                announce,
                created_by,
                creation_date,
                encoding,
                info,
            }
        )
    }

    pub fn announce_url(&self) -> Result<String, Error> {
        let mut announce_vec = self.announce.as_bytes().to_vec();

        for &byte in b"?info_hash=" {
            announce_vec.push(byte);
        }

        for &byte in self.info.sha1().as_bytes() {
            announce_vec.push(byte);
        }

        match str::from_utf8(announce_vec.as_slice()) {
            Ok(v) => Ok(v.to_string()),
            Err(_) => Err(Error::new("Failed to parse url.".to_string())),
        }
    }
}

impl fmt::Display for Torrent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        format(fmt, self)
    }
}

fn format(fmt: &mut fmt::Formatter, v: &Torrent) -> fmt::Result {
    write!(fmt, "Torrent: {{ ")?;
    write!(fmt, "announce: \"{}\", ", v.announce)?;
    write!(fmt, "created_by: \"{}\", ", v.created_by)?;
    write!(fmt, "creation_date: {}, ", v.creation_date)?;
    write!(fmt, "encoding: \"{}\", ", v.encoding)?;
    write!(fmt, "info: {} ", v.info)?;
    write!(fmt, "}}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencoding::decoder::decode;

    fn torrent(data: &[u8]) -> Result<Torrent, Error> {
        Torrent::from(
            decode(data.to_vec())
        )
    }

    fn assert_result_matches_error(msg: String, result: Result<Torrent, Error>) {
        let actual = match result {
            Ok(_) => panic!("Unexpected Ok value."),
            Err(e) => e,
        };
        assert_eq!(msg, format!("{}", actual));
    }

    #[test]
    fn test_err_when_input_is_not_a_dictionary() {
        let result = torrent(b"le");
        assert_result_matches_error("Bencode is not a dict.".to_string(), result);
    }

    #[test]
    fn test_err_when_input_is_an_empty_dictionary() {
        let result = torrent(b"de");
        assert_result_matches_error("\"announce\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_err_when_announce_is_present() {
        let result = torrent(b"d8:announce3:yese");
        assert_result_matches_error("\"created by\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_err_when_created_by_is_present() {
        let result = torrent(b"d8:announce3:yes10:created by5:dereke");
        assert_result_matches_error("\"encoding\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_err_when_encoding_is_present() {
        let result = torrent(b"d8:announce3:yes10:created by5:derek8:encoding5:UTF-8e");
        assert_result_matches_error("\"creation date\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_err_when_creation_date_is_present() {
        let result = torrent(b"d8:announce3:yes10:created by5:derek8:encoding5:UTF-813:creation datei170ee");
        assert_result_matches_error("\"info\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_ok_when_all_values_are_present() {
        let result = torrent(
            b"d8:announce3:yes10:created by5:derek8:encoding5:UTF-813:creation datei170e4:infod6:lengthi4e4:name5:derek12:piece lengthi100e6:pieces3:z\xc3\x287:privatei1eee"
        );

        let expected_info = TorrentInfo {
            length: 4,
            name: "derek".to_string(),
            piece_length: 100,
            private: true,
            pieces: vec![b'z', 195, 40],
        };

        let expected = Torrent {
            announce: "yes".to_string(),
            created_by: "derek".to_string(),
            encoding: "UTF-8".to_string(),
            creation_date: 170,
            info: expected_info,
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn test_announce_url_when_all_values_are_present() {
        let expected_info = TorrentInfo {
            length: 4,
            name: "derek".to_string(),
            piece_length: 100,
            private: true,
            pieces: vec![b'z', 195, 40],
        };

        let expected = Torrent {
            announce: "yes".to_string(),
            created_by: "derek".to_string(),
            encoding: "UTF-8".to_string(),
            creation_date: 170,
            info: expected_info,
        };

        assert_eq!("yes?info_hash=%3AJ%9A%B3%D7%3E%D0t%BDD%DDz%A5%EE%9D%DE%8C%AD%28%AE", expected.announce_url().unwrap())
    }
}
