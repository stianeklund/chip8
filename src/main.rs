// src/main.rs
extern crate sdl2;
extern crate sdl2_image;
extern crate rand;

use std::env;
mod cpu;
mod display;
use display::Display;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Load rom (TODO: check size)
    cpu.load_bin(bin);

    // Initialize SDL Window
    let mut display = Display::new(&sdl_context);

    // CPU execution cycle
    'step: loop {
        if cpu.draw_flag {
            cpu.step(&mut display);
            display.draw(&cpu.pixels);
            cpu.update_timers();
        }

        // Iterate over eventpump & wait for Esc
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                // TODO Implement keyboard
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'step,
                _ => {}
            }
        }
    }
}
