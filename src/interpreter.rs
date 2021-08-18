use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::font::FONT_SET;
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
    pub keys: [u8; 16],
}

impl Interpreter {
    pub fn new(rom: Vec<u8>, display: Canvas<Window>) -> Interpreter {
        let mut memory = [0; 4096];

        for (i, byte) in rom.iter().enumerate() {
            memory[i + 0x200] = *byte;
        }

        for (i, byte) in FONT_SET.iter().enumerate() {
            memory[i] = *byte;
        }

        Interpreter {
            regs: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory,
            stack: [0; 16],
            pc: 0x200,
            sp: 0,
            display,
            vram: [0; 2048],
            keys: [0; 16],
        }
    }

    pub fn draw(&mut self) {
        self.display.set_draw_color(Color::BLACK);
        self.display.clear();
        self.display.set_draw_color(Color::WHITE);

        for y in 0..32 {
            for x in 0..64 {
                if self.vram[y * 32 + x] == 1 {
                    self.display
                        .fill_rect(Rect::new(x as i32 * 16, y as i32 * 16, 16, 16))
                        .unwrap();
                }
            }
        }
    }
}
