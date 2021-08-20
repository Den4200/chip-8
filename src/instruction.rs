use crate::masks::{ooox, ooxo, ooxx, oxoo, oxxx, xooo};

pub type Address = u16;
pub type Byte = u8;
pub type Nibble = u8;

pub type IRegister = u16;
pub type Register = u8;

#[allow(non_camel_case_types)]
pub enum Instruction {
    OP_00E0,
    OP_00EE,
    OP_1NNN(Address),
    OP_2NNN(Address),
    OP_3XNN(Register, Byte),
    OP_4XNN(Register, Byte),
    OP_5XY0(Register, Register),
    OP_6XNN(Register, Byte),
    OP_7XNN(Register, Byte),
    OP_8XY0(Register, Register),
    OP_8XY1(Register, Register),
    OP_8XY2(Register, Register),
    OP_8XY3(Register, Register),
    OP_8XY4(Register, Register),
    OP_8XY5(Register, Register),
    OP_8XY6(Register),
    OP_8XY7(Register, Register),
    OP_8XYE(Register),
    OP_9XY0(Register, Register),
    OP_ANNN(Address),
    OP_BNNN(Address),
    OP_CXNN(Register, Byte),
    OP_DXYN(Register, Register, Nibble),
    OP_EX9E(Register),
    OP_EXA1(Register),
    OP_FX07(Register),
    OP_FX0A(Register),
    OP_FX15(Register),
    OP_FX18(Register),
    OP_FX1E(Register),
    OP_FX29(Register),
    OP_FX33(Register),
    OP_FX55(Register),
    OP_FX65(Register),
}

impl Instruction {
    pub fn new(v: u16) -> Option<Self> {
        match xooo(&v) {
            0x0 => match ooxx(&v) {
                0xE0 => Some(Instruction::OP_00E0),
                0xEE => Some(Instruction::OP_00EE),
                _ => None,
            },
            0x1 => Some(Instruction::OP_1NNN(oxxx(&v))),
            0x2 => Some(Instruction::OP_2NNN(oxxx(&v))),
            0x3 => Some(Instruction::OP_3XNN(oxoo(&v), ooxx(&v))),
            0x4 => Some(Instruction::OP_4XNN(oxoo(&v), ooxx(&v))),
            0x5 => Some(Instruction::OP_5XY0(oxoo(&v), ooxo(&v))),
            0x6 => Some(Instruction::OP_6XNN(oxoo(&v), ooxx(&v))),
            0x7 => Some(Instruction::OP_7XNN(oxoo(&v), ooxx(&v))),
            0x8 => match ooox(&v) {
                0x0 => Some(Instruction::OP_8XY0(oxoo(&v), ooxo(&v))),
                0x1 => Some(Instruction::OP_8XY1(oxoo(&v), ooxo(&v))),
                0x2 => Some(Instruction::OP_8XY2(oxoo(&v), ooxo(&v))),
                0x3 => Some(Instruction::OP_8XY3(oxoo(&v), ooxo(&v))),
                0x4 => Some(Instruction::OP_8XY4(oxoo(&v), ooxo(&v))),
                0x5 => Some(Instruction::OP_8XY5(oxoo(&v), ooxo(&v))),
                0x6 => Some(Instruction::OP_8XY6(oxoo(&v))),
                0x7 => Some(Instruction::OP_8XY7(oxoo(&v), ooxo(&v))),
                0xE => Some(Instruction::OP_8XYE(oxoo(&v))),
                _ => None,
            },
            0x9 => Some(Instruction::OP_9XY0(oxoo(&v), ooxo(&v))),
            0xA => Some(Instruction::OP_ANNN(oxxx(&v))),
            0xB => Some(Instruction::OP_BNNN(oxxx(&v))),
            0xC => Some(Instruction::OP_CXNN(oxoo(&v), ooxx(&v))),
            0xD => Some(Instruction::OP_DXYN(oxoo(&v), ooxo(&v), ooox(&v))),
            0xE => match ooxx(&v) {
                0x9E => Some(Instruction::OP_EX9E(oxoo(&v))),
                0xA1 => Some(Instruction::OP_EXA1(oxoo(&v))),
                _ => None,
            },
            0xF => match ooxx(&v) {
                0x07 => Some(Instruction::OP_FX07(oxoo(&v))),
                0x0A => Some(Instruction::OP_FX0A(oxoo(&v))),
                0x15 => Some(Instruction::OP_FX15(oxoo(&v))),
                0x18 => Some(Instruction::OP_FX18(oxoo(&v))),
                0x1E => Some(Instruction::OP_FX1E(oxoo(&v))),
                0x29 => Some(Instruction::OP_FX29(oxoo(&v))),
                0x33 => Some(Instruction::OP_FX33(oxoo(&v))),
                0x55 => Some(Instruction::OP_FX55(oxoo(&v))),
                0x65 => Some(Instruction::OP_FX65(oxoo(&v))),
                _ => None,
            },
            _ => None,
        }
    }
}
