// version 1 header:
// 0       4       8               16              24              32
// +-------+-------+---------------+---------------+---------------+
// | type  | ver   | extension     | connection_id                 |
// +-------+-------+---------------+---------------+---------------+
// | timestamp_microseconds                                        |
// +---------------+---------------+---------------+---------------+
// | timestamp_difference_microseconds                             |
// +---------------+---------------+---------------+---------------+
// | wnd_size                                                      |
// +---------------+---------------+---------------+---------------+
// | seq_nr                        | ack_nr                        |
// +---------------+---------------+---------------+---------------+

use std::fmt;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Header {
    packet_type: u8,
    version: u8,
    extension: u8,
    connection_id: u16,
    timestamp_microseconds: u32,
    timestamp_difference_microseconds: u32,
    wnd_size: u32,
    seq_nr: u16,
    ack_nr: u16,
}

impl Header {
    pub fn new(packet_type: u8, version: u8, extension: u8, connection_id: u16, timestamp_microseconds: u32, timestamp_difference_microseconds: u32, wnd_size: u32, seq_nr: u16, ack_nr: u16) -> Self {
        Self { 
            packet_type,
            version,
            extension,
            connection_id,
            timestamp_microseconds,
            timestamp_difference_microseconds,
            wnd_size,
            seq_nr,
            ack_nr,
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "0       4       8               16              24              32")?;
        writeln!(fmt, "+-------+-------+---------------+---------------+---------------+")?;
        writeln!(fmt, "| {:#x}   | {:#x}   | {:#04x}          | {:#06x}                        |", self.packet_type, self.version, self.extension, self.connection_id)?;
        writeln!(fmt, "+-------+-------+---------------+---------------+---------------+")?;
        writeln!(fmt, "| {:#010x}                                                    |", self.timestamp_microseconds)?;
        writeln!(fmt, "+---------------+---------------+---------------+---------------+")?;
        writeln!(fmt, "| {:#010x}                                                    |", self.timestamp_difference_microseconds)?;
        writeln!(fmt, "+---------------+---------------+---------------+---------------+")?;
        writeln!(fmt, "| {:#010x}                                                    |", self.wnd_size)?;
        writeln!(fmt, "+---------------+---------------+---------------+---------------+")?;
        writeln!(fmt, "| {:#06x}                        | {:#06x}                        |", self.seq_nr, self.ack_nr)?;
        writeln!(fmt, "+---------------+---------------+---------------+---------------+")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let header = Header {
            packet_type: 0,
            version: 1,
            extension: 0,
            connection_id: 42,
            timestamp_microseconds: 1997,
            timestamp_difference_microseconds: 3,
            wnd_size: 4048,
            seq_nr: 5,
            ack_nr: 3,
        };
        let expected = "\
             0       4       8               16              24              32\n\
             +-------+-------+---------------+---------------+---------------+\n\
             | 0x0   | 0x1   | 0x00          | 0x002a                        |\n\
             +-------+-------+---------------+---------------+---------------+\n\
             | 0x000007cd                                                    |\n\
             +---------------+---------------+---------------+---------------+\n\
             | 0x00000003                                                    |\n\
             +---------------+---------------+---------------+---------------+\n\
             | 0x00000fd0                                                    |\n\
             +---------------+---------------+---------------+---------------+\n\
             | 0x0005                        | 0x0003                        |\n\
             +---------------+---------------+---------------+---------------+\n";
        assert_eq!(expected, format!("{}", header));
    }
}
