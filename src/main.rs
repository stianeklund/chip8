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
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    // Load rom (TODO: check size)
    cpu.load_bin(bin);

    // Initialize SDL Window
    let mut display = Display::new(&sdl_context);

    // Frame timing
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    // CPU execution cycle
    'step: loop {
        // Timing second instance
        let now = timer.ticks();
        let dt = now - before;
        // let elapsed = dt as f64 / 1_000.0;

            if dt < interval {
                timer.delay(interval - dt);
                if DEBUG {
                    println!("Time elapsed since last frame is too small");
                }
                continue;
                before = now;
                fps += 1;
            if now - last_second < 1_000 {
                println!("FPS: {}", fps);
            }
            last_second = now;
            fps = 0;
        }
        cpu.step(&mut display);
        display.draw(&cpu.pixels);
        cpu.update_timers();

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
