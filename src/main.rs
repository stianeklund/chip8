// src/main.rs
use std::env;
use cpu::Cpu;
mod cpu;

fn main() {
    // Workaround
    let args: Vec<String> = env::args().collect();
	  if args.len() != 2 {
		    println!("[PATH_TO_ROM]");
		    return;
	  }
	  let bin = &args[1];

    let mut cpu = Cpu::new();
    cpu.load_bin(bin);

    loop {
    cpu.step();
    cpu.update_timers();
    }
}
