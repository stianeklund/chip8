// src/main.rs
use std::{env, fs};
use std::io::Read;
use std::path::Path;
use cpu::Cpu;
mod cpu;

fn main() {
    let file_name = env::args().nth(1).expect("Missing input file");
    let rom = Cpu::read_rom(file_name);
    let mut cpu = Cpu::new();
}
