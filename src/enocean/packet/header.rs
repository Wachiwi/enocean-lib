use super::types;

pub struct Header {
    pub data_length: u16,
    pub optional_length: u8,
    pub packet_type: u8,
}


