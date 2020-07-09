use std::fmt;
use std::net::Ipv4Addr;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl fmt::Display for Peer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}:{}", self.ip, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let peer = Peer { ip: Ipv4Addr::new(192, 168, 2, 11), port: 6881 };
        assert_eq!("192.168.2.11:6881", peer.to_string());
    }

    #[test]
    fn test_display() {
        let peer = Peer { ip: Ipv4Addr::new(192, 168, 2, 11), port: 6881 };
        assert_eq!("192.168.2.11:6881", format!("{}", peer));
    }

    #[test]
    fn test_debug() {
        let peer = Peer { ip: Ipv4Addr::new(192, 168, 2, 11), port: 6881 };
        assert_eq!("Peer { ip: 192.168.2.11, port: 6881 }", format!("{:?}", peer));
    }
}
