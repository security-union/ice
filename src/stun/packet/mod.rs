pub mod attribute;
pub mod error_code;
pub mod header;

pub mod address;

pub use self::address::{Address, Family};
pub use self::attribute::{Attribute, AttributeType};
pub use self::error_code::ErrorCode;
pub use self::header::{Class, Header, Method};

#[derive(Debug)]
pub struct Packet {
    header: Header,
    attributes: Vec<Attribute>,
}

impl Packet {
    pub fn new(header: Header) -> Result<Self, &'static str> {
        Ok(Packet {
            header: header,
            attributes: Vec::new(),
        })
    }
    pub fn from_bytes(&self) -> Result<Self, &'static str> {
        unimplemented!();
    }
    pub fn into_bytes(&self) -> &[u8] {
        unimplemented!();
    }
    pub fn to_hex_string(&self) -> String {
        unimplemented!();
    }
}
