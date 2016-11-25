// src/cpu.rs
use std::{fs, env};
use std::io::Read;
use std::path::Path;

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
    memory: [u8; 4096],
    v: [u8; 16],
    i: isize,
    pc: usize,
    stack: [u16; 16],
    sp: usize,
    dt: u8,
    st: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            opcode: 0,
            memory: [0; 4096],       // 0x000 - 0xFFF. 0x000 - 0x1FF for interpreter
            v: [0; 16],              // 8-bit general purpose register, (V0 - VE*).
            i: 0x200,                // Index register (start at 0x200)
            pc: 0x200,               // Program Counter. Jump to 0x200 on RST
            stack: [0; 16],          // Interpreter returns to value when done with subroutine
            sp: 0,                   // Stack pointer. Used to point to topmost level of the Stack
            dt: 0,                   // 8-bit Delay Timer
            st: 0,                   // 8-bit Sound timer
                                     // * VF is a special register used to store overflow bit
        };
        cpu
    }

    // TODO: Handle case where rom is larger than memory space
    // Load rom into memory at 0x200
    pub fn read_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
        let mut file = fs::File::open(path).unwrap();
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).expect("Reading rom failed");
        // Return file_buf
        file_buf
    }

    // Emulate cycle & read the next opcode from memory
    pub fn emulate_cycle(&mut self) {
        // Read two bytes
        self.opcode = (self.memory[self.pc] as u16) << 8 |
        (self.memory[self.pc + 1] as u16);
        println!("{:04x}: {:04x}", self.pc, self.opcode);

        let nnn_reg: u16 = (self.opcode & 0x0FFF) as u16;
        let kk_reg: u8 = (self.opcode & 0x00FF) as u8;
        // Vx & Vy register identifiers.
        // Bitshift right to get raw value (we want 0x4 instead of 0x40)
        let x_reg: u8 = (self.opcode & 0x0F00 >> 8) as u8;
        let y_reg: u8 = (self.opcode & 0x00F0 >> 4) as u8;
        // TODO: Handle Chip-8 instructions
    }
}
