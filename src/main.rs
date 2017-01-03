// src/main.rs
extern crate sdl2;
extern crate sdl2_image;
extern crate rand;

use std::env;
mod cpu;
mod display;
mod keypad;
use std::thread::sleep;
use std::time::Duration;

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

    // SDL2 context
    let sdl_context = sdl2::init().expect("sdl2 init failed in main");
    let mut timer = sdl_context.timer().unwrap();

    // Load rom (TODO: check size)
    cpu.load_bin(bin);

    // Initialize Keyboard
    let mut keypad = keypad::Keypad::new(&sdl_context);

    // Initialize SDL Window
    let mut display = Display::new(&sdl_context);

    // Set tread sleep time
    let ms = Duration::from_millis(1);

    // CPU execution cycle
    'step: loop {

        match keypad.key_press(&mut cpu.keypad) {
            keypad::State::Exit => break 'step,
            keypad::State::Continue => {}
        }

        cpu.run(&mut display);
        display.draw(&cpu.pixels);
        cpu.update_timers();

        sleep(ms);
        }
}
