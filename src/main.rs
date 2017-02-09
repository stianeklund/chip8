extern crate sdl2;
extern crate rand;

mod cpu;
mod display;
mod keypad;

use std::env;
use display::Display;

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
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;
    // Changable step speed
    let mut speed = cpu.speed;

    // CPU execution cycle
    'run: loop {
        match keypad.key_press(&mut cpu.keypad) {
            keypad::State::Exit => break 'run,
            keypad::State::Continue => {}
            keypad::State::Debug => { cpu.mode.debug = true; println!("Debug enabled");}
            keypad::State::StopDebug => { cpu.mode.normal= true; cpu.mode.debug = false; println!("Debug off"); }
            keypad::State::Increase => { speed = speed.wrapping_add(1); println!("Speed: {}", speed); }
            keypad::State::Decrease => { speed = speed.wrapping_sub(1); println!("Speed: {}", speed); }
        }
        // Execute & decode opcodes 2 times for every time we loop
        cpu.step(speed, &mut display);

        // Frame timing
        let now = timer.ticks();
        let dt = now - before;

        if dt < interval { timer.delay(interval - dt); continue; }
        before = now;
        fps += 1;

        if now - last_second > 1000 {
            // if DEBUG { println!("FPS: {}", fps); }
            last_second = now;
            fps = 0;
        }
        cpu.update_timers(dt as f32);
    }
}
