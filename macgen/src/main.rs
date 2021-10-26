use std::{fmt::Display, os::linux::fs::MetadataExt};

use rand::RngCore;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let octet = &self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress {
    pub fn new() -> Self {
        let mut octets = [8_u8; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0011; // local and unicast
        Self(octets)
    }

    fn is_local(&self) -> bool {
        MacAddress::is_set(self.0[0], 1)
    }

    fn is_unicast(&self) -> bool {
        MacAddress::is_set(self.0[0], 0)
    }

    fn is_set(b: u8, k: u8) -> bool {
        (b >> k) & 1 == 1
    }
}

fn main() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("{}", mac);
}
