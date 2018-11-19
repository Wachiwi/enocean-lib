extern crate crc_any;

use crc_any::{CRC};

use super::header::Header;

pub struct Packet {
    sync_byte: u8,
    header: Header,
    crc8_header: u8,
    data: Vec<u8>,
    optional_data: Option<Vec<u8>>,
    crc8_data: u8,
}

pub trait FromBytes {
    fn from_bytes(bytes: Vec<u8>) {}
}


impl Packet {
    pub fn verify(&self) -> bool {
        let header_data_left = (self.header.data_length & 0x00FF) as u8;
        let header_data_right = (self.header.data_length.swap_bytes() & 0x00FF) as u8;

        /*
        let buff: [u8; 4] = [header_data_left, header_data_right, self.header.optional_length, self.header.packet_type];
        let mut crc8 = crc8::Crc8::create_lsb(0x10);
        let crc = crc8.calc(&buff, 4, 0);
        println!("eq?: 0x{:0X} <=> 0x{:0X}", self.crc8_header, crc);


        let mut crc8 = CRC::crc8();

        crc8.digest(b"hello");

        assert_eq!([236, 83, 136, 71, 154, 124, 145, 63].to_vec(), crc64ecma.get_crc_vec());


        assert_eq!(self.crc8_header, crc);
        */
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::crc_any::{CRC};

    #[test]
    fn test_u16_to_u8() {
        let high: u16 = 0x3525;

        let right = high & 0x00FF;
        let left = high.swap_bytes() & 0x00FF;

        assert_eq!(right, 0x25);
        assert_eq!(left, 0x35);

        let right_u8: u8 = right as u8;
        let left_u8: u8 = left as u8;

        assert_eq!(left_u8, 0x35);
        assert_eq!(right_u8, 0x25);
    }

    #[test]
    fn test_verify_packet() {
        let valid_packet = Packet {
            sync_byte: 0x55,
            header: Header {
                data_length: 0x07,
                optional_length: 0x07,
                packet_type: 0x01,
            },
            crc8_header: 0x7A,
            data: vec![0xF6, 0x50, 0xFE, 0xFD, 0x17, 0x78, 0x30],
            optional_data: Some(vec![0x2, 0xFF, 0xFF, 0xFF, 0xFF, 0x44, 0x0]),
            crc8_data: 0xD6,
        };


        let mut crc8_header = CRC::crc8atm();
        crc8_header.digest(valid_packet.header.to_vec());
        let mut crc8_data = CRC::crc8atm();
        crc8_data.digest(valid_packet.data);

        assert_eq!([0x7A].to_vec(), crc8_header.get_crc_vec());


        //assert_eq!(valid_packet.verify(), true);
    }
}
