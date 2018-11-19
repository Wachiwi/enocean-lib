pub trait ToU8 {
    fn to_u8_vec(&self) -> Vec<u8>;
    fn to_u8_tuple(&self) -> (u8, u8);
}

impl ToU8 for u16 {
    fn to_u8_vec(&self) -> Vec<u8> {
        let high: u16 = *self;

        let right = (high & 0x00FF) as u8;
        let left = (high.swap_bytes() & 0x00FF) as u8;

        return vec![left, right];
    }

    fn to_u8_tuple(&self) -> (u8, u8) {
        let high: u16 = *self;

        let right= (high & 0x00FF) as u8;
        let left = (high.swap_bytes() & 0x00FF) as u8;

        return (left, right);
    }
}

#[cfg(test)]
mod tests {


}