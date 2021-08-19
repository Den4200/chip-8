use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::font::FONT_SET;
use crate::instruction::{Address, Byte, IRegister, Instruction, Register};

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
    pub fn new(rom: Vec<u8>, display: Canvas<Window>) -> Self {
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

        self.display.present();
    }

    pub fn tick(&mut self, keys: [u8; 16]) {
        self.keys = keys;

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        let opcode = self.get_optcode();
        match Instruction::new(opcode as u16) {
            Some(instruction) => self.run_instruction(&instruction),
            None => println!("unknown instruction"),
        };
    }

    fn get_optcode(&self) -> u16 {
        let pc = self.pc as usize;
        (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16)
    }

    fn pc_next(&mut self) {
        self.pc += 2;
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        let mut should_increment = true;

        match instruction {
            Instruction::OP_00E0 => {
                for index in 0..self.vram.len() {
                    self.vram[index] = 0;
                }
            }
            Instruction::OP_00EE => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            Instruction::OP_1NNN(nnn) => {
                self.pc = *nnn;
                should_increment = false;
            }
            Instruction::OP_2NNN(nnn) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = *nnn;
                should_increment = false;
            }
            Instruction::OP_3XNN(vx, nn) => {
                if self.regs[*vx as usize] == *nn {
                    self.pc_next();
                }
            }
            Instruction::OP_4XNN(vx, nn) => {
                if self.regs[*vx as usize] != *nn {
                    self.pc_next();
                }
            }
            Instruction::OP_5XY0(vx, vy) => {
                if self.regs[*vx as usize] == self.regs[*vy as usize] {
                    self.pc_next();
                }
            }
            Instruction::OP_6XNN(vx, nn) => {
                self.regs[*vx as usize] = *nn;
            }
            Instruction::OP_7XNN(vx, nn) => {
                let index = *vx as usize;
                self.regs[index] = self.regs[index].wrapping_add(*nn);
            }
            Instruction::OP_8XY0(vx, vy) => {
                self.regs[*vx as usize] = self.regs[*vy as usize];
            }
            Instruction::OP_8XY1(vx, vy) => {
                self.regs[*vx as usize] |= self.regs[*vy as usize];
            }
            Instruction::OP_8XY2(vx, vy) => {
                self.regs[*vx as usize] &= self.regs[*vy as usize];
            }
            Instruction::OP_8XY3(vx, vy) => {
                self.regs[*vx as usize] ^= self.regs[*vy as usize];
            }
            Instruction::OP_8XY4(vx, vy) => {
                let x = self.regs[*vx as usize];
                let y = self.regs[*vy as usize];

                self.regs[*vx as usize] = x.wrapping_add(y);
                self.regs[0xF] = (x as u16 + y as u16 > u8::MAX as u16) as u8;
            }
            Instruction::OP_8XY5(vx, vy) => {
                let x = self.regs[*vx as usize];
                let y = self.regs[*vy as usize];

                self.regs[*vx as usize] = x.wrapping_sub(y);
                self.regs[0xF] = (x > y) as u8;
            }
            Instruction::OP_8XY6(vx, vy) => {
                self.regs[*vx as usize] = self.regs[*vy as usize] >> 1;
                self.regs[0xF] = self.regs[*vy as usize] & 1;
            }
            Instruction::OP_8XY7(vx, vy) => {
                let x = self.regs[*vx as usize];
                let y = self.regs[*vy as usize];

                self.regs[*vx as usize] = y.wrapping_sub(x);
                self.regs[0xF] = (y > x) as u8;
            }
            Instruction::OP_8XYE(vx, vy) => {
                self.regs[*vx as usize] = self.regs[*vy as usize] << 1;
                self.regs[0xF] = self.regs[*vy as usize] >> 7;
            }
            Instruction::OP_9XY0(vx, vy) => {
                if self.regs[*vx as usize] != self.regs[*vy as usize] {
                    self.pc_next();
                }
            }
            Instruction::OP_ANNN(nnn) => {
                self.i = *nnn;
            }
            Instruction::OP_BNNN(nnn) => {
                self.pc = *nnn + self.regs[0] as u16;
                should_increment = false;
            }
            Instruction::OP_CXNN(vx, nn) => {
                self.regs[*vx as usize] = rand::random::<u8>() & *nn;
            }
            Instruction::OP_DXYN(vx, vy, n) => {
                self.regs[0xF] = 0;

                for byte in 0..*n {
                    let y = (self.regs[*vy as usize] + byte) as u16 % 32;

                    for bit in 0..8 {
                        let x = (self.regs[*vx as usize] + bit) as u16 % 64;
                        let color = (self.memory[(self.i + byte as u16) as usize] >> (7 - bit)) & 1;

                        self.regs[0xF] |= color & self.vram[(y * 32 + x) as usize];
                        self.vram[(y * 32 + x) as usize] ^= color;
                    }
                }
            }
            Instruction::OP_EX9E(vx) => {
                if self.keys[self.regs[*vx as usize] as usize] == 1 {
                    self.pc_next();
                }
            }
            Instruction::OP_EXA1(vx) => {
                if self.keys[self.regs[*vx as usize] as usize] != 1 {
                    self.pc_next();
                }
            }
            Instruction::OP_FX07(vx) => {
                self.regs[*vx as usize] = self.delay_timer;
            }
            Instruction::OP_FX0A(vx) => {
                if self.keys[self.regs[*vx as usize] as usize] != 1 {
                    should_increment = false;
                }
            }
            Instruction::OP_FX15(vx) => {
                self.delay_timer = self.regs[*vx as usize];
            }
            Instruction::OP_FX18(vx) => {
                self.sound_timer = self.regs[*vx as usize];
            }
            Instruction::OP_FX1E(vx) => {
                self.i += self.regs[*vx as usize] as u16;
            }
            Instruction::OP_FX29(vx) => {
                self.i = (self.regs[*vx as usize] * 5) as u16;
            }
            Instruction::OP_FX33(vx) => {
                let x = self.regs[*vx as usize];
                let i = self.i as usize;

                self.memory[i] = x / 100;
                self.memory[i + 1] = x / 10 % 10;
                self.memory[i + 2] = x % 10;
            }
            Instruction::OP_FX55(vx) => {
                for index in 0..=*vx as usize {
                    self.memory[self.i as usize + index] = self.regs[index];
                    self.i += 1;
                }
            }
            Instruction::OP_FX65(vx) => {
                for index in 0..=*vx as usize {
                    self.regs[index] = self.memory[self.i as usize + index];
                    self.i += 1;
                }
            }
        };

        if should_increment {
            self.pc_next();
        }
    }
}
