extern crate sdl2;
extern crate rand;

use std::env;

mod cpu;
mod display;
mod keypad;

#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[Path to rom]");
        return;
    }
    let bin = &args[1];
    let mut cpu = cpu::Cpu::new();

    // SDL2 context
    let sdl_context = sdl2::init().expect("sdl2 init failed in main");
    let mut timer = sdl_context.timer().expect("sdl context timer failed");

    // Load rom
    cpu.load_bin(bin);
    let mut keypad = keypad::Keypad::new(&sdl_context);
    let mut display = display::Display::new(&sdl_context);

    // Frame timing
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    'run: loop {
        match keypad.key_press(&mut cpu.keypad) {
            keypad::State::Exit => break 'run,
            keypad::State::Continue => {}

            // TODO Enable & disable debug mode with the same key..
            keypad::State::Debug => {
                cpu.mode.debug = true != cpu.mode.debug;
                println!("Debug:{}", cpu.mode.debug as bool);
            }
            keypad::State::Increase => {
                cpu.speed = cpu.speed.wrapping_add(1);
                println!("Speed: {}", cpu.speed);
            }
            keypad::State::Decrease => {
                cpu.speed = cpu.speed.wrapping_sub(1);
                println!("Speed: {}", cpu.speed);
            }
            keypad::State::Reset => { cpu.reset(); }
        }
        // Execute & decode opcodes 2 times for every time we loop
        cpu.step(cpu.speed, &mut display);

        // Frame timing
        let now = timer.ticks();
        let dt = now - before;

        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }
        before = now;
        fps += 1;

        if now - last_second > 1000 {
            last_second = now;
            fps = 0;
        }

        cpu.update_timers(dt as f32);
    }
}
