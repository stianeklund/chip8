// src/main.rs
use std::{env, fs};
use std::io::Read;
use std::path::Path;
use cpu::Cpu;

mod cpu;

fn main() {
    let file_name = env::args().nth(1).expect("Missing input file");
    // TODO: Handle case where rom is larger than memory space
    pub fn read_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
        let mut file = fs::File::open(path).unwrap();
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).expect("Reading rom failed");
        // Return file_buf
        file_buf
    }

    let mut cpu = cpu::Cpu::new();
    read_rom(file_name);
    cpu.step();
    // cpu.update_timers();
}
