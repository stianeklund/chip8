// src/cpu.rs
use std::{fs, env};
use std::io::Read;
use std::path::Path;

// TODO: Implement fmt::Debug for Cpu
const DEBUG: bool = true;

// Load built-in fonts into memory
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0x20, 0x60, 0x20, 0x20, 0x70, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F

];

// TODO: Implement input & display handling with SDL2
pub struct Cpu {
    opcode: u16,
    memory: [u8; 4096],     // 0x000 - 0xFFF. 0x000 - 0x1FF for interpreter
    v: [u8; 16],            // 8-bit general purpose register, (V0 - VE*).
    i: isize,               // Index register (start at 0x200)
    pc: usize,              // Program Counter. Jump to 0x200 on RST
    stack: [u16; 16],       // Interpreter returns to value when done with subroutine
    sp: usize,              // Stack pointer. Used to point to topmost level of the Stack
    delay_timer: u8,        // 8-bit Delay Timer
    sound_timer: u8,        // 8-bit Sound Timer
    draw_flag: bool,        // 0x00E0 CLS
    display: [u8; 64 * 32], // Display is an array of 64x32 pixels
    keypad: [u16; 16]       // Keypad is HEX based(0x0-0xF)
    // * VF is a special register used to store overflow bit
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut memory: [u8; 4096] = [0; 4096];

        for i in 0..80 {
            memory[i] = FONT[i];
        }

        Cpu {
            opcode: 0,
            memory: memory,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            draw_flag: true,
            display: [0; 64 * 32],
            keypad: [0; 16]
        }
    }

    // TODO: Handle case where rom is larger than memory space
    pub fn read_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
        let mut file = fs::File::open(path).unwrap();
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).expect("Reading rom failed");
        // Return file_buf
        file_buf
    }

    // TODO: Implement delta time to keep track of timers so that they update every 60s.
    // Update delay & sound timers (decrement delay & sound until they're 0)
    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        if self.sound_timer > 0 {
            println!("Beep!");
        }
    }

    // This is big-endian, so we need to shift 8 bytes to the left
    // then bitwise-or it with the next byte to get the full 16-bit value

    // Read in 2 bytes
    pub fn step(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        // All instructions are 2 bytes long & are stored most-significant-byte first
        let n: u16 = (self.opcode & 0x000F) as u16;   // nibble 4 bit value
        let nnn: u16 = (self.opcode & 0x0FFF) as u16; // addr 12-bit value
        let kk: u8 = (self.opcode & 0x00FF) as u8;    // byte 8-bit value

        // Decode Vx & Vy register identifiers.
        let x: u8 = (self.opcode & 0x0F00 >> 8) as u8; // Bitshift right to get 0x4
        let y: u8 = (self.opcode & 0x00F0 >> 4) as u8; // Original value is 0x40

        println!("Executing opcode 0x{:04x}", self.opcode);
        //println!("Executing opcode: 0x{:X}", self.opcode);

        // Execute instructions, might need to use 0x0000
        match (self.opcode & 0xF00) {
            0x0000 => match kk {
                // 00E0 CLS
                0xE0 => {
                    // Null out the array (Set all pixels to 0)
                    self.display = [0; 64 * 32];
                    self.draw_flag = true;
                    self.pc += 2; // increment PC by 2
                },
                // 0xEE (00EE) RET Return from a subroutine
                // The interpreter should set the pc to the address at the top
                // of the stack then subtract 1 from the SP
                0xEE => {
                    self.sp -= 1;
                    // self.pc = self.stack[(self.sp as usize)];
                    self.pc = self.stack[self.sp] as usize;

                },
                _ => panic!("Panicked at: 0x{:04x}", self.opcode),
            },
            _ => panic!("Panicked at: 0x{:04x}", self.opcode),
        }
    }
}
