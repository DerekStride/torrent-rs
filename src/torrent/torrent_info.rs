use std::fmt;
use std::result::Result;
use sha1::Digest;
use percent_encoding;

use crate::bencoding;
use crate::bencoding::byte_string::ByteString;
use crate::bencoding::bencode::Bencode;

use crate::torrent::error::Error;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TorrentInfo {
    pub length: i64,
    pub name: String,
    pub piece_length: i64,
    pub private: bool,
    pub pieces: Vec<u8>,
}

impl TorrentInfo {
    pub fn from(input: Bencode) -> Result<Self, Error> {
        let length = input.get_number("length")?;
        let name = input.get_string("name")?;
        let piece_length = input.get_number("piece length")?;
        let private_num = input.get_number("private")?;
        let private = private_num == 1;
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

    pub fn sha1(&self) -> String {
        let mut hasher = sha1::Sha1::new();
        let encoded_info = bencoding::encoder::encode(self.torrent_info());
        hasher.input(encoded_info);
        let sha1 = hasher.result();

        percent_encoding::percent_encode(sha1.as_slice(), percent_encoding::NON_ALPHANUMERIC).to_string()
    }

    fn torrent_info(&self) -> Bencode {
        let mut dict = bencoding::bencode::DictMap::new();
        dict.insert(
            ByteString::from_str("length"),
            Bencode::Number(self.length),
        );
        dict.insert(
            ByteString::from_str("name"),
            Bencode::ByteString(self.name.as_bytes().to_vec()),
        );
        dict.insert(
            ByteString::from_str("piece length"),
            Bencode::Number(self.piece_length),
        );

        let private = if self.private { 1 } else { 0 };
        dict.insert(
            ByteString::from_str("private"),
            Bencode::Number(private),
        );

        let mut v = vec![0; self.pieces.len()];
        v.copy_from_slice(&self.pieces);

        dict.insert(
            ByteString::from_str("pieces"),
            Bencode::ByteString(v),
        );

        return Bencode::Dict(dict)
    }
}

impl fmt::Display for TorrentInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        format(fmt, self)
    }
}

fn format(fmt: &mut fmt::Formatter, v: &TorrentInfo) -> fmt::Result {
    write!(fmt, "TorrentInfo: {{ ")?;
    write!(fmt, "name: \"{}\", ", v.name)?;
    write!(fmt, "length: {}, ", v.length)?;
    write!(fmt, "piece_length: {}, ", v.piece_length)?;
    write!(fmt, "private: {}, ", v.private)?;
    write!(fmt, "pieces: {:?} ", v.pieces)?;
    write!(fmt, "}}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencoding::decoder::decode;

    fn torrent_info(data: &[u8]) -> Result<TorrentInfo, Error> {
        TorrentInfo::from(
            decode(data.to_vec())
        )
    }

    fn assert_result_matches_error(msg: String, result: Result<TorrentInfo, Error>) {
        let actual = match result {
            Ok(_) => panic!("Unexpected Ok value."),
            Err(e) => e,
        };
        assert_eq!(msg, format!("{}", actual));
    }

    #[test]
    fn test_err_when_input_is_not_a_dictionary() {
        let result = torrent_info(b"le");
        assert_result_matches_error("Bencode is not a dict.".to_string(), result);
    }

    #[test]
    fn test_err_when_input_is_an_empty_dictionary() {
        let result = torrent_info(b"de");
        assert_result_matches_error("\"length\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_err_when_length_is_present() {
        let result = torrent_info(b"d6:lengthi4ee");
        assert_result_matches_error("\"name\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_err_when_name_is_present() {
        let result = torrent_info(b"d6:lengthi4e4:name5:dereke");
        assert_result_matches_error("\"piece length\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_err_when_piece_length_is_present() {
        let result = torrent_info(b"d6:lengthi4e4:name5:derek12:piece lengthi100ee");
        assert_result_matches_error("\"private\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_err_when_private_is_present() {
        let result = torrent_info(b"d6:lengthi4e4:name5:derek12:piece lengthi100e7:privatei1ee");
        assert_result_matches_error("\"pieces\" key is not present in torrent file.".to_string(), result);
    }

    #[test]
    fn test_ok_when_all_values_are_present() {
        let data = b"d6:lengthi4e4:name5:derek12:piece lengthi100e7:privatei1e6:pieces3:z\xc3\x28e";
        let result = torrent_info(data);

        let expected = TorrentInfo {
            length: 4,
            name: "derek".to_string(),
            piece_length: 100,
            private: true,
            pieces: vec![b'z', 195, 40],
        };

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn test_sha_when_all_values_are_present() {
        let data = b"d6:lengthi4e4:name5:derek12:piece lengthi100e6:pieces3:z\xc3\x287:privatei1ee";
        let mut hasher = sha1::Sha1::new();
        hasher.input(data.to_vec());
        let sha1 = hasher.result();
        let expected_str = percent_encoding::percent_encode(sha1.as_slice(), percent_encoding::NON_ALPHANUMERIC).to_string();

        let torrent = TorrentInfo {
            length: 4,
            name: "derek".to_string(),
            piece_length: 100,
            private: true,
            pieces: vec![b'z', 195, 40],
        };

        assert_eq!("%3AJ%9A%B3%D7%3E%D0t%BDD%DDz%A5%EE%9D%DE%8C%AD%28%AE", expected_str);
        assert_eq!(expected_str, torrent.sha1());
    }
}
