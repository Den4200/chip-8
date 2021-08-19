use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod font;
mod input;
mod instruction;
mod interpreter;
mod masks;

use input::Input;
use interpreter::Interpreter;

const CPU_TICK: Duration = Duration::from_millis(2);
const FRAME_TICK: Duration = Duration::from_millis(16);

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

    let mut cpu_last = Instant::now();
    let mut frame_last = Instant::now();
    loop {
        if frame_last.elapsed() >= FRAME_TICK {
            interpreter.draw();
            frame_last = Instant::now();
        }

        if cpu_last.elapsed() >= CPU_TICK {
            let keys = match input.poll() {
                Some(keys) => keys,
                None => break,
            };

            interpreter.tick(keys);

            cpu_last = Instant::now();
        } else {
            sleep(CPU_TICK - cpu_last.elapsed());
        }
    }
}
