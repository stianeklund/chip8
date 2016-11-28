// src/main.rs
extern crate sdl2;
extern crate sdl2_image;
extern crate rand;

use std::env;
use cpu::Cpu;

mod cpu;
mod display;

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
