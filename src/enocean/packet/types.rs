enum PacketType {
    RadioErp1,
    Response,
    RadioSubTel,
    Event,
    CommonCommand,
    SmartAckCommand,
    RemoteManCommand,
    RadioMessage,
    RadioErp2,
    Radio802154,
    Command24,
    Reserved,
    Unknown,
    Custom,
}


impl PacketType {
    pub fn get_value(&self) -> u8 {
        return match *self {
            PacketType::RadioErp1 => 0x01,
            PacketType::Response => 0x02,
            PacketType::RadioSubTel => 0x03,
            PacketType::Event => 0x04,
            PacketType::CommonCommand => 0x05,
            PacketType::SmartAckCommand => 0x06,
            PacketType::RemoteManCommand => 0x07,
            PacketType::RadioMessage => 0x09,
            PacketType::RadioErp2 => 0x0A,
            PacketType::Radio802154 => 0x10,
            PacketType::Command24 => 0x11,
        };
    }

    /*
    pub fn from_byte(byte: u8) -> Result<PacketType, E> {
        return match byte {
            0x00 => PacketType::Reserved,
            0x01 => PacketType::RadioErp1,
            0x02 => PacketType::Response,
            0x03 => PacketType::RadioSubTel,
            0x04 => PacketType::Event,
            0x05 => PacketType::CommonCommand,
            0x06 => PacketType::SmartAckCommand,
            0x07 => PacketType::RemoteManCommand,
            0x08 => PacketType::Reserved,
            0x09 => PacketType::RadioMessage,
            0x0A => PacketType::RadioErp2,
            0x0B..0x0F => PacketType::Reserved,
            0x10 => PacketType::Radio802154,
            0x11 => PacketType::Command24,
            0x12..0x7F => PacketType::Reserved,
            0x80..0xFF => PacketType::Custom,
            _ => PacketType::Unknown
        }
    }
    */
}