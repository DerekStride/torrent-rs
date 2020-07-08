use std::result::Result;

use crate::bencoding::bencode::Bencode;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TorrentInfo {
    pub length: i64,
    pub name: String,
    pub piece_length: i64,
    pub private: bool,
    pub pieces: Vec<u8>,
}

impl TorrentInfo {
    pub fn from(input: Bencode) -> Result<Self, String> {
        let length = input.get_number("length")?;
        let name = input.get_string("name")?;
        let piece_length = input.get_number("piece length")?;
        let private_str = input.get_string("private")?;
        let private = private_str.as_str() == "1";
        let pieces = input.remove_bytestring("pieces")?;

        Ok(
            Self {
                length,
                name,
                piece_length,
                private,
                pieces,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencoding::decoder::decode;

    fn torrent_info(data: &[u8]) -> Result<TorrentInfo, String> {
        TorrentInfo::from(
            decode(data.to_vec())
        )
    }

    #[test]
    fn test_err_when_input_is_not_a_dictionary() {
        let result = torrent_info(b"le");
        assert_eq!(Err("Bencode is not a dict.".to_string()), result);
    }

    #[test]
    fn test_err_when_input_is_an_empty_dictionary() {
        let result = torrent_info(b"de");
        assert_eq!(Err("\"length\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_length_is_present() {
        let result = torrent_info(b"d6:lengthi4ee");
        assert_eq!(Err("\"name\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_name_is_present() {
        let result = torrent_info(b"d6:lengthi4e4:name5:dereke");
        assert_eq!(Err("\"piece length\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_piece_length_is_present() {
        let result = torrent_info(b"d6:lengthi4e4:name5:derek12:piece lengthi100ee");
        assert_eq!(Err("\"private\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_err_when_private_is_present() {
        let result = torrent_info(b"d6:lengthi4e4:name5:derek12:piece lengthi100e7:private1:1e");
        assert_eq!(Err("\"pieces\" key is not present in torrent file.".to_string()), result);
    }

    #[test]
    fn test_ok_when_all_values_are_present() {
        let result = torrent_info(
            b"d6:lengthi4e4:name5:derek12:piece lengthi100e6:pieces3:z\xc3\x287:private1:1e"
        );

        let expected = TorrentInfo {
            length: 4,
            name: "derek".to_string(),
            piece_length: 100,
            private: true,
            pieces: vec![b'z', 195, 40],
        };

        assert_eq!(Ok(expected), result);
    }
}
