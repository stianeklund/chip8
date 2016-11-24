
use std::env;
use std::fs;
use std::io::Read;
use cpu::Cpu;

mod cpu;

fn main() {

    let file_name = env::args().nth(1).expect("Missing input file");
    let mut file = fs::File::open(&file_name).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).expect("Reading rom failed");
    // Create a new immutable binding
    let file_buf = file_buf;

    let mut cpu = Cpu::new();

}
