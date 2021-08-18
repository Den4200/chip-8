use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::instruction::{Address, Byte, IRegister, Register};

pub struct Interpreter {
    pub regs: [Register; 16],
    pub i: IRegister,
    
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub memory: [Byte; 4096],
    pub stack: [Address; 16],

    pub pc: Address,
    pub sp: u8,

    pub display: Canvas<Window>,
    pub vram: [u8; 2048],
    pub keys: [bool; 16],
}
