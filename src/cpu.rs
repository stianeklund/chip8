// src/cpu.rs
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand;
use rand::Rng;
use display::Display;

// Load built-in fonts into memory
// Ref: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.4
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

pub struct Cpu {
    opcode: u16,
    memory: [u8; 4096],             // 0x000 - 0xFFF. 0x000 - 0x1FF for interpreter
    v: [u8; 16],                    // 8-bit general purpose register, (V0 - VE*).
    i: u16,                         // Index register (start at 0x200)
    pc: u16,                        // Program Counter. Jump to 0x200 on RST
    stack: [u16; 16],               // Interpreter returns to value when done with subroutine
    sp: u16,                        // Stack pointer
    delay_timer: u8,                // 8-bit Delay Timer
    sound_timer: u8,                // 8-bit Sound Timer
    pub draw_flag: bool,            // 0x00E0 CLS
    pub pixels: [[bool; 64 as usize]; 32 as usize], // For rendering
    pub keypad: [u16; 16]           // Keypad is HEX based(0x0-0xF)
    // * VF is a special register used to store overflow bit
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut memory: [u8; 4096] = [0; 4096];

        // load sprites into memory
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
            pixels: [[false; 64 as usize]; 32 as usize],

            // display: [0; 64 * 32],
            //display: Display::new(&sdl_context),
            keypad: [0; 16]
        }
    }

    pub fn load_bin(&mut self, file: &str) {
        let path = Path::new(file);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => panic!("Unable to open file, "),
        };

        let mut buf = Vec::new();
        match file.read_to_end(&mut buf) {
            Ok(buf) => buf,
            Err(e) => panic!("Oh no: {}", e),
        };
        let buf_size = buf.len();
        for i in 0..buf_size {
            self.memory[i + 512] = buf[i];
        }
    }
    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;

        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("Beep!");
            }
            self.sound_timer -= 1;
        }
    }

    /// This is big-endian, so we need to shift 8 bytes to the left
    /// then bitwise-or it with the next byte to get the full 16-bit value
    /// All instructions are 2 bytes long & are stored most-significant-byte first
    /// One opcode is 2 bytes long. Fetch 2 bytes and merge them
    pub fn step(&mut self, display: &mut Display) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 |
        (self.memory[self.pc as usize + 1] as u16);

        // Decode Vx & Vy register identifiers.
        let x = ((self.opcode & 0x0F00) as usize) >> 8; // Bitshift right to get 0x4
        let y = ((self.opcode & 0x00F0) as u8) >> 4; // Original value is 0x40
        // let n = (self.opcode & 0x000F) as u8;        // nibble 4-bit value

        let nn = self.opcode & 0x00FF;               // 8 bit constant u16
        let nnn = self.opcode & 0x0FFF;              // addr 12-bit value
        let kk = self.opcode & 0x00FF;               // u8, byte 8-bit value


        // println!("PC is: {:X}", self.pc);
        println!("PC: 0x0{:X}  |  Opcode: {:X}  | Index Register: {:X}",
                 self.pc, self.opcode, self.i);

        // TODO: Move opcodes into separate method
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x000F {

                // 00E0 CLS
                0x0000 => {
                    // Set all pixels to 0
                    display.clear();
                    self.draw_flag = true;
                    self.pc += 2; // increment PC by 2
                },

                // Set pc to address at the top of the stack then subtract 1 from SP
                0x000E => {
                    self.sp -= 1;
                    self.pc = self.stack[(self.sp as usize)];
                    self.pc += 2;
                },
                _ => println!("Unknown upcode: {:X}", self.opcode),
            },

            // 1NNN Jump to location
            0x1000 => {
                self.pc = nnn;
            },
            // 2NNN Call subroutine at nnn
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            // 3XKK Skip next instruction if Vx = kk
            0x3000 => {
                if self.v[x as usize] == kk as u8 {
                    self.pc += 4;
                }
                self.pc += 2;
            },
            // 4XKK Skip next instruction if Vx != kk
            0x4000 => {
                if self.v[x as usize] != kk as u8 {
                    self.pc += 4;
                }
                self.pc += 2;
            },
            // 5XY0 Skip next instruction if Vx = Vy
            0x5000 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 4;
                }
                self.pc += 2;
            },
            // 6XKK Set Vx = kk. Put value of kk in to Vx register
            0x6000 => {
                self.v[x as usize] == kk as u8;
                self.pc += 2; // Add for test
            },
            // 7XKK Add value kk to Vx
            0x7000 => {
                // Wrapping (modular) addition, prevents add overflow
                self.v[x] = self.v[x].wrapping_add(nn as u8) as u8;
                self.pc += 2;
            },

            // 8XY0 Set Vx = Vy
            0x8000 =>  match self.opcode & 0x000F {
                0x0000 => {
                    self.v[x as usize] = self.v[y as usize];
                    self.pc += 2;
                },
                // 8XY1 Set Vx to Vx & Vy
                0x0001 => {
                    self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
                    self.pc += 2;
                },
                // 8XY3 Set Vx to XOR Vy
                0x0003 => {
                    self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
                    self.pc += 2;
                },
                // 8XY4 Set Vx = Vx + Vy, set VF = carry
                0x0004 => {
                    if self.v[y as usize] > (0xFF - self.v[x as usize]) {
                        self.v[0xF]  = 1; // Set carry (also used for pixel flip)
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[x as usize] += self.v[y as usize];
                    self.pc += 2;
                },
                // 8XY5 Set Vx = Vx - Vy, set VF = NOT borrow
                0x0005 => {
                    if self.v[x as usize] > self.v[y as usize] {
                        self.v[0xF] = 1; // VF set to not borrow
                    } else {
                        self.v[0xF] = 0;
                    }
                },
                // 8XY6 Vx = Vx Shift right by 1 If the least-significant bit of
                // Vx is 1 then VF is set to 1, otherwise 0. Then Vx is divided by 2
                0x0006 => {
                    self.v[0xF] = self.v[x as usize] & 0x1;
                    self.v[x as usize] >>= 1;
                    self.pc += 2;
                },
                // 8XY7 Set Vx = Vy - Vx, VF NOT borrow
                0x0007 => {
                    if self.v[y as usize] > self.v[x as usize] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.pc +=2;
                },
                // 8XYE
                // If the most-significant bit of Vx is 1 then VF is set to 1
                // Otherwise VF is set to 0 and Vx is multiplied by 2.
                0x000E => {
                    self.v[0xF] = self.v[x as usize] >> 7;
                    self.v[x as usize] <<= 1;
                },
                _ => println!("Unknown opcode [0x8000], {:X}", self.opcode),
            },
            // 9XY0 Skip next instruction if Vx != Vy
                0x9000 => {
                    if self.v[x as usize] != self.v[y as usize] {
                        self.pc += 4;
                    }
                    self.pc += 2;
                },
                // ANNN Set I to the address of NNN
                0xA000 => {
                    self.i = nnn;
                    self.pc += 2;
                },
            // BNNN Jump to address NNN + V0
            0xB000 => {
                self.pc = nnn + self.v[0x0] as u16;
            },
            // CXNN Set Vx to a random number masked by NN
            0xC000 => {
                let mut rng = rand::thread_rng();
                let random_number: u8 = rng.gen::<u8>();
                self.v[x as usize] = random_number as u8 & nn as u8;
                self.pc += 2;
            },
            // DXYN Draw to display
            0xD000 => {
                // READ: http://devernay.free.fr/hacks/chip8/2.4

                // TODO: Move into spearate function
                let x_index = self.v[(self.opcode << 4 >> 12) as usize] as usize;
                let y_index = self.v[(self.opcode << 8 >> 12) as usize] as usize;
                let height = (self.opcode << 12 >> 12) as usize;

                let mut flipped = false;

                for y in 0..height {
                    let row = self.memory[self.i as usize + y];
                    for x in 0..8 {
                        if row & ((0x80 >> x as u8)) != 0 {
                            flipped |= self.pixels[(y_index + y) % 32]
                                [(x_index + x) % 64] as bool;
                            self.pixels[(y_index + y) % 32]
                                [(x_index + x) % 64] ^= true;
                            self.v[0xF] = flipped as u8;
                            display.draw_flag = true;
                        }
                    }
                }
                self.pc += 2;
            },
            0xE000 => match self.opcode & 0x00FF {
                // EX9E Skip next instruction if key stored in Vx is pressed
                // Usually the next instruction is JMP to skip to a code block
                0x009E => {
                    if self.keypad[self.v[x as usize] as usize] != 0 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                },
                // EXA1 Skip next instruction if key stored in Vx isn't pressed
                0x00A1 => {
                    if self.keypad[self.v[x as usize] as usize] != 1 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                },
                _ => println!("Unknown opcode {:02X}", self.opcode),
            },
            0xF000 => match self.opcode & 0x00FF {
                // FX15 Set delay timer to Vx
                0x0007 => {
                    self.v[x as usize] = self.delay_timer;
                    self.pc += 2;
                },
                // FX0A Key press awaited then stored in Vx
                // All instructions halted until next key event
                0x000A => {
                    // TODO: Implement keypress
                    let mut keypad_press = false;
                    // Keypad is 0 - 16 values
                    for i in 0..16 {
                        if self.keypad[i] != 0 { // key is pressed
                            self.v[x as usize] = i as u8;
                            keypad_press = true;
                        }
                    }
                    if !keypad_press  {
                        return;
                    }
                    self.pc +=2;
                },
                // FX15 Set delay timer
                0x0015 => {
                    self.delay_timer = self.v[x as usize];
                    self.pc += 2;
                },
                // FX18 Set sound timer
                0x0018 => {
                    self.sound_timer = self.v[x as usize];
                    self.pc += 2;
                },

                //FX1E Add Vx to I (MEM)
                0x001E => {
                    self.i = self.i.wrapping_add(self.v[x as usize] as u16);
                    self.pc += 2;
                },

                // FX29 Set I to the location of the sprite for char in Vx
                // Chars 0-F are represented by a 4x5 font
                // Each character contains 5 elements
                // Create 0x5 font accessible in memory
                0x0029 => {
                    self.i = (self.v[x as usize] * 0x5) as u16;
                    self.pc += 2;
                },
                // FX33 (BCD) The interpreter takes the decimal value of Vx
                // & places the hundreds digit in memory at location in I,
                // the tens digit at location I+1, and the ones digit at location I+2.

                0x0033 => {
                    // TODO  Decimal representation
                    let i = self.i as usize;
                    self.memory[i] = self.v[x as usize] / 100;
                    self.memory[i + 1] = (self.v[x as usize] / 10) % 10;
                    self.memory[i + 2] = (self.v[x as usize]  % 100) % 10;
                },
                // FX55 Stores V0 to VX in memory starting at I
                // TODO
                0x0055 => {
                    for x in 0..(x + 1) {
                        self.memory[self.v[x] as usize] = self.memory[self.v[x] as usize + x];
                    }
                         self.pc += 2;
                },
                // FX65 Fills V0 to VX with values from memory starting at I
                // TODO
                0x0065 => {
                    for x in 0..(x + 1) {
                        self.memory[self.v[x] as usize] = self.memory[self.v[x] as usize + x];
                    }
                    self.pc += 2;
                },
                _ => println!("Unknown opcode: {}", self.opcode),
            },
            _ => println!("Unknown opcode: {}", self.opcode),
        };
    }
}


