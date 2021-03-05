use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PacketType {
    STData = 0,
    STFin = 1,
    STState = 2,
    STReset = 3,
    STSyn = 4,
}

impl PacketType {
    pub fn from(i: u8) -> PacketType {
        match i {
            0 => PacketType::STData,
            1 => PacketType::STFin,
            2 => PacketType::STState,
            3 => PacketType::STReset,
            4 => PacketType::STSyn,
            _ => panic!("invalid option {}", i)
        }
    }
}

impl fmt::Display for PacketType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:#x}", *self as u8)
    }
}
