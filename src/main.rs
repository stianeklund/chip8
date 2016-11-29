// src/main.rs
extern crate sdl2;
extern crate sdl2_image;
extern crate rand;

use std::env;
mod cpu;
mod display;
use display::Display;
const DEBUG: bool = false;

fn main() {

    let args: Vec<String> = env::args().collect();
	  if args.len() != 2 {
		    println!("[Path to rom]");
		    return;
	  }

	  let bin = &args[1];

    // Initialize CPU
    let mut cpu = cpu::Cpu::new();
    let sdl_context = sdl2::init().expect("sdl2 init failed in main");

    // Load rom
    cpu.load_bin(bin);

    // Initialize SDL Window
    let mut window = Display::new(&sdl_context);
    // Lazy debugging..
    if DEBUG {
        cpu.step();
        cpu.update_timers();
    } else {
        'step: loop {
            // if draw_flag is set do
            if cpu.draw_flag {
                cpu.step();
                cpu.update_timers();
            }
        }
    }
}
