use std::result::Result;

use crate::bencoding::bencode::Bencode;
use crate::torrent::torrent_info::TorrentInfo;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Torrent {
    pub announce: String,
    pub created_by: String,
    pub creation_date: i64,
    pub encoding: String,
    pub info: TorrentInfo,
}

impl Torrent {
    pub fn from(input: Bencode) -> Result<Self, String> {        
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencoding::decoder::decode;

    fn torrent(data: &[u8]) -> Result<Torrent, String> {
        Torrent::from(
            decode(data.to_vec())
        )
    }

    #[test]
    fn test_err_when_input_is_not_a_dictionary() {
        let result = torrent(b"le");
        assert_eq!(Err("Bencode is not a dict.".to_string()), result);
    }

    #[test]
    fn test_err_when_input_is_an_empty_dictionary() {
        let result = torrent(b"de");
        assert_eq!(Err("\"announce\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_announce_is_present() {
        let result = torrent(b"d8:announce3:yese");
        assert_eq!(Err("\"created by\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_created_by_is_present() {
        let result = torrent(b"d8:announce3:yes10:created by5:dereke");
        assert_eq!(Err("\"encoding\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_encoding_is_present() {
        let result = torrent(b"d8:announce3:yes10:created by5:derek8:encoding5:UTF-8e");
        assert_eq!(Err("\"creation date\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_creation_date_is_present() {
        let result = torrent(b"d8:announce3:yes10:created by5:derek8:encoding5:UTF-813:creation datei170ee");
        assert_eq!(Err("\"info\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_ok_when_all_values_are_present() {
        let result = torrent(
            b"d8:announce3:yes10:created by5:derek8:encoding5:UTF-813:creation datei170e4:infod6:lengthi4e4:name5:derek12:piece lengthi100e6:pieces3:z\xc3\x287:private1:1ee"
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
}
