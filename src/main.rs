use std::env;
use std::fs::File;
use std::io::{Error, Read};
mod font;
mod input;
mod instruction;
mod interpreter;
mod masks;

use input::Input;
use interpreter::Interpreter;

fn load_rom(filename: &String) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filename)?;
    let mut rom = Vec::new();

    file.read_to_end(&mut rom)?;
    Ok(rom)
}

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("CHIP-8", 1024, 512)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    let mut input = Input::new(&sdl_context);

    let filename = env::args().nth(1).expect("missing filename argument");
    let rom = match load_rom(&filename) {
        Ok(rom) => rom,
        Err(err) => {
            println!("error reading {}: {}", filename, err);
            exit(1);
        }
    };

    let mut interpreter = Interpreter::new(rom, canvas);
}
