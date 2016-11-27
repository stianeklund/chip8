// src/main.rs
use std::env;
use cpu::Cpu;
mod cpu;
const DEBUG: bool = false;

fn main() {
    // Workaround
    let args: Vec<String> = env::args().collect();
	  if args.len() != 2 {
		    println!("[Path to rom]");
		    return;
	  }
	  let bin = &args[1];

    let mut cpu = Cpu::new();
    cpu.load_bin(bin);
    // Lazy debugging..
    if DEBUG {
        cpu.step();
        cpu.update_timers();
    } else {
        loop {
            cpu.step();
            cpu.update_timers();
        }
    }
}
