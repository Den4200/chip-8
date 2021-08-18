pub fn xooo(value: &u16) -> u8 {
    (value >> 12 & 0xF) as u8
}

pub fn oxoo(value: &u16) -> u8 {
    (value >> 8 & 0xF) as u8
}

pub fn ooxo(value: &u16) -> u8 {
    (value >> 4 & 0xF) as u8
}

pub fn ooox(value: &u16) -> u8 {
    (value & 0xF) as u8
}

pub fn ooxx(value: &u16) -> u8 {
    (value & 0xFF) as u8
}

pub fn oxxx(value: &u16) -> u16 {
    value & 0xFFF
}
