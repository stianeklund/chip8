extern crate sdl2;
extern crate sdl2_image;
extern crate rand;

mod cpu;
mod display;
mod keypad;

use std::env;
use display::Display;

use cpu::DEBUG;

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
    let mut timer = sdl_context.timer().expect("sdl_context timer failed");

    // Load rom
    cpu.load_bin(bin);

    // Initialize Keyboard
    let mut keypad = keypad::Keypad::new(&sdl_context);

    // Initialize SDL Window
    let mut display = Display::new(&sdl_context);

    // Frame timing
    let interval = 1000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    // CPU execution cycle
    'run: loop {
        match keypad.key_press(&mut cpu.keypad) {
            keypad::State::Exit => break 'run,
            keypad::State::Continue => {}
        }

        let now = timer.ticks();
        let dt: f32 = (now - before) as f32;
        let elapsed = dt as f32 / 1000.0;

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            if DEBUG { println!("FPS: {}", fps); }
            last_second = now;
            fps = 0;

        }
        cpu.run(&mut display);
        cpu.update_timers();
        cpu.step_instruction(dt);
    }
}
