use std::fs::File;
use std::io::{Error, Read};
mod font;
mod instruction;
mod interpreter;
mod masks;

fn load_rom(filename: &String) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filename)?;
    let mut rom = Vec::new();

    file.read_to_end(&mut rom)?;
    Ok(rom)
}
fn main() {
    println!("Hello, world!");
}
