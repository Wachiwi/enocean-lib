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

        let buff: [u8; 4] = [self.header.data_length.to_i8().to_usize(), self.header.data_length.swap_bytes().to_i8().to_usize(), self.header.optional_length, self.header.packet_type];
        let mut crc8 = crc8::Crc8::create_lsb(130);
        let crc = crc8.calc(&buff, 4, 0);
        println!("crc: {:?}", crc);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify() {
        let packet = Packet {
            sync_byte: 0x55,
            header: Header {
                data_length: 0,
                optional_length: 0,
                packet_type: 0x04,
            },
            crc8_header: 0,
            data: Vec::new(),
            optional_data: None,
            crc8_data: 0,
        };

        assert_eq!(packet.verify(), true);
    }
}
