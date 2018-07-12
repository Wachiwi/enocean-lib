extern crate crc8;

use super::header::Header;

pub struct Packet {
    sync_byte: u8,
    header: Header,
    crc8_header: u8,
    data: Vec<u8>,
    optional_data: Option<Vec<u8>>,
    crc8_data: u8,
}


impl Packet {
    pub fn verify(&self) -> bool {

        let header_data_left = (self.header.data_length & 0x00FF) as u8;
        let header_data_right = (self.header.data_length.swap_bytes() & 0x00FF) as u8;

        let buff: [u8; 4] = [header_data_left, header_data_right, self.header.optional_length, self.header.packet_type];
        let mut crc8 = crc8::Crc8::create_lsb(0x07);
        let crc = crc8.calc(&buff, 4, 0);

        println!("eq?: 0x{:0X} <=> 0x{:0X}", self.crc8_header, crc);

        assert_eq!(self.crc8_header, crc);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16_to_u8() {
        let high : u16 = 0x3525;

        let right = high & 0x00FF;
        let left  = high.swap_bytes() & 0x00FF;

        assert_eq!(right, 0x25);
        assert_eq!(left, 0x35);

        let right_u8 : u8 = right as u8;
        let left_u8 : u8 = left as u8;

        assert_eq!(left_u8, 0x35);
        assert_eq!(right_u8, 0x25);
    }

    #[test]
    fn test_verify() {
        let valid_packet = Packet {
            sync_byte: 0x55,
            header: Header {
                data_length: 0x3542,
                optional_length: 0x00,
                packet_type: 0x04,
            },
            crc8_header: 0xD6,
            data: Vec::new(),
            optional_data: None,
            crc8_data: 0,
        };

        assert_eq!(valid_packet.verify(), true);
    }
}
