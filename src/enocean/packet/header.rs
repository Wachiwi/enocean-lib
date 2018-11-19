use util::ToU8;

pub struct Header {
    pub data_length: u16,
    pub optional_length: u8,
    pub packet_type: u8,
}

impl Header {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut header_vec : Vec<u8> = self.data_length.to_u8_vec();
        header_vec.push(self.optional_length);
        header_vec.push(self.packet_type);
        return header_vec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_vec_works() {
        let header = Header {
            data_length: 0x07,
            optional_length: 0x07,
            packet_type: 0x01,
        };

        assert_eq!(header.to_vec(), vec![0x00, 0x07, 0x07, 0x01])
    }

}