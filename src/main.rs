
use std::{env, fs};
use std::io::Read;
use std::path::Path;
use cpu::Cpu;
mod cpu;

fn main() {
    let file_name = env::args().nth(1).expect("Missing input file");
    let rom = read_rom(file_name);
}
// Load ROM into memory at 0x200
fn read_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).expect("Reading rom failed");
    // Return file_buf
    file_buf
}
