extern crate sdl2;
extern crate sdl2_image;
extern crate rand;

mod cpu;
mod display;
mod keypad;

use std::env;
use display::Display;

pub const DEBUG: bool = true;

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
    let mut timer = sdl_context.timer().expect("sdl context timer failed");

    // Load rom
    cpu.load_bin(bin);

    // Initialize Keyboard
    let mut keypad = keypad::Keypad::new(&sdl_context);

    // Initialize SDL Window
    let mut display = Display::new(&sdl_context);

    // Frame timing
    let interval = 1_000 / 300;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;


    // CPU execution cycle
    'run: loop {
        match keypad.key_press(&mut cpu.keypad) {
            keypad::State::Exit => break 'run,
            keypad::State::Continue => {}
        }

        cpu.run(&mut display);
        cpu.update_timers();

        // Frame timing
        let now = timer.ticks();
        let dt = now - before;

        // Hacky.. delay if deltatime is smaller than interval
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }
        before = now;
        fps += 1;
        if now - last_second > 1000 {
            if DEBUG { println!("FPS: {}", fps); }
            last_second = now;
            fps = 0;
        }
        cpu.step_instruction(dt as f32);
    }
}
