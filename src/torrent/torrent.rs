use std::result::Result;

use crate::bencoding::bencode::Bencode;
use crate::torrent::torrent_info::TorrentInfo;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Torrent {
    pub announce: String,
    pub created_by: String,
    pub creation_date: i64,
    pub encoding: String,
    // pub info: TorrentInfo,
}

impl Torrent {
    pub fn from(input: Bencode) -> Result<Self, String> {        
        let announce = input.get_string("announce")?;
        let created_by = input.get_string("created_by")?;
        let encoding = input.get_string("encoding")?;
        let creation_date = input.get_number("creation_date")?;
        // let info = TorrentInfo::from(input.get("info")?)?;
        
        Ok(
            Self {
                announce,
                created_by,
                creation_date,
                encoding,
                // info,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencoding::decoder::decode;

    #[test]
    fn test_err_when_input_is_not_a_dictionary() {
        let raw = b"le".to_vec();
        let input = decode(raw);
        let result = Torrent::from(input);

        assert_eq!(Err("Bencode is not a dict.".to_string()), result);
    }
}
