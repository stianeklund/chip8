use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;
use rand;
use rand::Rng;

use crate::display::Display;
use crate::display::{WIDTH, HEIGHT, DisplayMode};

#[derive(Debug, Clone, Copy)]
pub struct Mode {
    pub debug: bool,
}

// CHIP-8 Fonts
const FONT: [u8; 240] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xe0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0x80, // C
    0xF0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    // Super CHIP fonts
    0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, // 0
    0x18, 0x78, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, // 1
    0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // 2
    0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 3
    0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0x03, 0x03, // 4
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 5
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 6
    0xFF, 0xFF, 0x03, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x18, // 7
    0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 8
    0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 9
    0x7E, 0xFF, 0xC3, 0xC3, 0xC3, 0xFF, 0xFf, 0xC3, 0xC3, 0xC3, // A
    0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, // B
    0x3C, 0xFF, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0xFF, 0x3C, // C
    0xFC, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, 0xFC, // D
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // E
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xC0, 0xC0, // F
];

pub struct Cpu {
    opcode: u16,
    memory: Box<[u8; 4096]>,       // 0x000 - 0xFFF. 0x000 - 0x1FF for interpreter
    v: [u8; 16],                   // 8-bit general purpose register, (V0 - VE*).
    i: u16,                        // Index register (start at 0x200)
    pc: u16,                       // Program Counter. Jump to 0x200 on RST
    stack: [u16; 16],              // Interpreter returns to value when done with subroutine
    sp: u16,                       // Stack pointer
    delay_timer: u8,               // 8-bit Delay Timer
    sound_timer: u8,               // 8-bit Sound Timer
    snd_tick: f32,                 // Sound timer tick
    tick: f32,                     // Cpu timer tick
    rpl_flags: [u8; 8],            // RPL User Flags (Used by opcodes FX75 & FX85)
    pub pixels: [[bool; WIDTH]; HEIGHT],
    pub keypad: [u8; 16],          // Keypad is HEX based(0x0-0xF)
    pub mode: Mode,                // Mode to turn on & off debugging
    pub display_mode: DisplayMode, // Normal & Extended display modes
    pub speed: u8,                 // CPU clock speed
    draw_flag: bool,               // Whether or not to redraw
    // *VF is a special register used to store overflow bit
}


impl Cpu {
    pub fn new() -> Cpu {
        let mut memory = [0; 4096];

        for i in 0..240 {
            memory[i] = FONT[i];
        }

        Cpu {
            opcode: 0,
            memory: Box::new(memory),
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            snd_tick: 0.0,
            tick: 0.0,
            rpl_flags: [0; 8],
            pixels: [[false; WIDTH]; HEIGHT],
            keypad: [0; 16],
            mode: Mode { debug: false },
            display_mode: DisplayMode::Normal,
            speed: 2,
            draw_flag: false,
        }
    }

    pub fn load_bin(&mut self, file: &str) {
        let path = Path::new(file);
        let mut file = File::open(&path).expect("File open failed");
        let mut buf = Vec::new();

        file.read_to_end(&mut buf).expect("Failed to read file");

        if buf.len() >= 3585 { panic!("ROM is too large, size: {}", buf.len()); }

        let buf_len = buf.len();
        for i in 0..buf_len { self.memory[i + 512] = buf[i]; }
    }

    pub fn update_timers(&mut self, dt: f32) {
        if self.delay_timer > 0 {
            self.tick -= dt;

            if self.tick <= 0.0 {
                self.delay_timer -= 1;
                self.tick = 1.0 / 60.0;
            }
        }

        if self.sound_timer > 0 {
            self.snd_tick -= dt;
            if self.mode.debug { println!("BEEP"); }

            if self.snd_tick <= 0.0 {
                self.sound_timer -= 1;
                self.snd_tick = 1.0 / 60.0;
            }
        }
    }

    pub fn reset(&mut self) {
        println!("Reset interpreter");

        for v in &mut self.v {
            *v = 0;
        }

        self.i = 0;
        self.pc = 0x200;
        self.sp = 0;

        self.delay_timer = 0;
        self.sound_timer = 0;

        self.pixels = [[false; WIDTH]; HEIGHT];
        self.draw_flag = true;
    }


    // Fetch high & low bytes & merge
    pub fn run(&mut self, display: &mut Display) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 |
            (self.memory[self.pc as usize + 1] as u16);

        // Decode Vx & Vy register identifiers.
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        let nnn: u16 = (self.opcode & 0x0FFF) as u16;    // addr 12-bit value
        let kk: u8 = (self.opcode & 0x00FF) as u8;      // u8, byte 8-bit value

        if self.mode.debug == true {
            println!("Opcode: {:X} | PC: {:#?} | SP: {:X} | I: {:X} | V0: {} | \
                      V1: {} | V2: {} | V3: {} | V4: {} | V5: {} | \
                      V6: {} | V7: {} | V8: {} | V9: {} | VA: {} | \
                      VB: {} | VC: {} | VD: {} | VE: {} | VF: {}",
                     self.opcode, self.pc, self.sp, self.i, self.v[0], self.v[1],
                     self.v[2], self.v[3], self.v[4], self.v[5], self.v[6], self.v[7],
                     self.v[8], self.v[9], self.v[10], self.v[11], self.v[12],
                     self.v[13], self.v[14], self.v[15]);
        }


        // Relying on the first 4 bits is not enough in this case.
        // We need to compare the last four bits, hence the second match block.
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x00F0 {

                    // 00CN SCHIP Scroll down N lines
                    0x00C0 => {
                        let n = (self.opcode & 0x000F) as usize;
                        let extended = self.display_mode == DisplayMode::Extended;
                        let i = if extended { n } else { n / 2 };

                        // Subtract n from y to scroll down n height
                        for x in 0..WIDTH {
                            for y in (1..HEIGHT).rev() {
                                self.pixels[y][x] = self.pixels[y - i][x];
                            }
                            for y in 0..i {
                                self.pixels[y][x] = false;
                            }
                        }

                        display.draw(&self.pixels, 10, 10);

                        self.pc += 2;
                        self.draw_flag = true;

                        if self.mode.debug { println!("Scroll down N lines"); }
                    }

                    _ => match self.opcode & 0x00FF {

                        // 00E0 (CLS) Clear screen
                        0x00E0 => {
                            self.pixels = [[false; WIDTH]; HEIGHT];
                            self.draw_flag = true;
                            self.pc += 2;
                        }

                        // 00EE (RET) Return from subroutine call
                        0x00EE => {
                            self.sp -= 1;
                            self.pc = self.stack[(self.sp as usize)];
                            self.pc += 2;
                        }

                        // 00FB (SCHIP) Scroll screen 4 pixels right
                        0x00FB => {
                            let extended = self.display_mode == DisplayMode::Extended;
                            let i = if extended { 4 } else { 2 };

                            for y in 0..HEIGHT {
                                for x in (i..WIDTH).rev() {
                                    self.pixels[y][x] = self.pixels[y][x - i];
                                }

                                for x in 0..i {
                                    self.pixels[y][x] = false;
                                }
                            }

                            display.draw(&self.pixels, 10, 10);

                            self.draw_flag = true;
                            self.pc += 2;

                            if self.mode.debug { println!("Scroll 4 pixels right"); }
                        }

                        // 00FC (SCHIP) Scroll screen 4 pixels left
                        0x00FC => {
                            let extended = self.display_mode == DisplayMode::Extended;
                            let i = if extended { 4 } else { 2 };

                            for y in 0..HEIGHT {
                                for x in 0..(WIDTH - i) {
                                    self.pixels[y][x] = self.pixels[y][x + i];
                                }

                                for x in (WIDTH - i)..WIDTH {
                                    self.pixels[y][x] = false;
                                }
                            }

                            display.draw(&self.pixels, 10, 10);

                            self.draw_flag = true;
                            self.pc += 2;

                            if self.mode.debug { println!("Scroll 4 pixels left"); }
                        }

                        // 00FE (SCHIP) Disable extended screen mode
                        0x00FE => {
                            self.display_mode = DisplayMode::Normal;
                            self.pc += 2;

                            if self.mode.debug { println!("(Extended mode disabled"); }
                        }

                        // 00FD (SCHIP) Exit CHIP8 Interpreter
                        0x00FD => {

                            // Reset instead of exiting interpreter
                            self.reset()
                        }

                        // 00FF (SCHIP) Enabled extended screen mode: 128 x 64
                        0x00FF => {
                            self.display_mode = DisplayMode::Extended;
                            if self.mode.debug { println!("Extended mode enabled"); }

                            self.pc += 2;
                        }

                        0x0000 => {
                            return;
                        }
                        _ => println!("Unknown opcode: 00{:X}", self.opcode),
                    }
                }
            }

            // 1NNN Jump to location
            0x1000 => {
                self.pc = nnn;
            }

            // 2NNN Call suboutine at address nnn
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;

                // Stack size is 16 so we need to wrap
                self.sp = self.sp.wrapping_add(1);
                self.pc = nnn;
            }

            // 3XKK Skip next instruction if Vx = kk
            0x3000 => {
                if self.v[x] == kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 4XKK Skip next instruction if Vx != kk
            0x4000 => {
                if self.v[x as usize] != kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 5XY0 Skip next instruction if Vx = Vy
            0x5000 => {
                if self.v[x] == self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 6XKK Set Vx = kk. Put value of kk in to Vx register
            0x6000 => {
                self.v[x] = kk;
                self.pc += 2;
            }

            // 7XKK Add value kk to Vx
            0x7000 => {
                // Wrapping addition, prevents add overflow
                self.v[x] = self.v[x].wrapping_add(kk);
                self.pc += 2;
            }

            // 8XY0 Set Vx = Vy
            0x8000 => {
                match self.opcode & 0x000F {
                    0x0000 => {
                        self.v[x] = self.v[y];
                        self.pc += 2;
                    }

                    // 8XY1 Set Vx to Vx OR Vy
                    0x0001 => {
                        self.v[x] |= self.v[y];
                        self.pc += 2;
                    }

                    // 8XY2 Set Vx to Vx AND Vy
                    0x0002 => {
                        self.v[x] &= self.v[y];
                        self.pc += 2;
                    }

                    // 8XY3 Set Vx to Vx XOR Vy
                    0x0003 => {
                        self.v[x] ^= self.v[y];
                        self.pc += 2;
                    }

                    // 8XY4 Set Vx = Vx + Vy, set VF = carry
                    0x0004 => {
                        let reg = (self.v[x] as u16) + (self.v[y] as u16);
                        self.v[x] = reg as u8;

                        // Set carry if the value of Vx + Vy is larger than 0xFF (255)
                        self.v[0xF] = (reg > 0xFF) as u8;
                        self.pc += 2;
                    }

                    // 8XY5 Set Vx = Vx - Vy, set VF = NOT borrow v[0xF]
                    0x0005 => {
                        let borrow = self.v[y] > self.v[x];

                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                        self.v[0xF] = if borrow { 0 } else { 1 };


                        self.pc += 2;
                    }

                    // 8XY6 Vx = Vx Shift right by 1 If the least-significant bit of
                    // Vx is 1 then VF is set to 1, otherwise 0.
                    0x0006 => {
                        self.v[0xF] = self.v[x] & 1;
                        self.v[x] >>= 1;

                        self.pc += 2;
                    }

                    // 8XY7 Set Vx = Vy - Vx, VF NOT borrow
                    0x0007 => {
                        let borrow = self.v[x] > self.v[y];

                        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                        self.v[0xF] = if borrow { 0 } else { 1 };


                        self.pc += 2;
                    }

                    // 8XYE
                    // If the most-significant bit of Vx is 1 then VF is set to 1
                    // Otherwise VF is set to 0 and Vx is multiplied by 2.
                    0x000E => {
                        self.v[0xF] = (self.v[x] >> 7) & 1;
                        self.v[x] <<= 1;
                        self.pc += 2;
                    }
                    _ => println!("Unknown opcode [0x8000], {:X}", self.opcode),
                }
            }

            // 9XY0 Skip next instruction if Vx != Vy
            0x9000 => {
                if self.v[x] != self.v[y] { self.pc += 4; } else { self.pc += 2; }
            }
            // ANNN Load index register (I) with NNN
            0xA000 => {
                self.i = nnn;
                self.pc += 2;
            }
            // BNNN Jump to address NNN + V0
            0xB000 => {
                self.pc = nnn.wrapping_add(self.v[0] as u16);
            }

            // CXNN Set Vx to a random number masked by kk
            0xC000 => {
                let mut rng = rand::thread_rng();
                let random_number: u8 = rng.gen_range(0, 255);
                self.v[x] = random_number & kk;

                self.pc += 2;
            }

            // DXYN Draw sprite starting at x, y, n specifies how many bytes the sprite is
            // Starting at memory location of self.i
            0xD000 => {
                let n = (self.opcode & 0x000F) as usize; // Sprite height

                // Let sprite width be 16 (for extended drawing mode)
                let extended = self.display_mode == DisplayMode::Extended;

                let w = if n == 0 && extended { 16 } else { 8 } as usize;
                let h = if n == 0 && extended { 16 } else { n } as usize;

                let sprite_x = self.v[x] as usize;
                let sprite_y = self.v[y] as usize;

                // Set collision flag
                self.v[0xF] = 0;

                for j in 0..h {
                    let row = self.memory[(self.i as usize + j as usize)] as u16;

                    // TODO: 16x16 sprites are only rendered in half for some reason
                    for i in 0..w {
                        let xi = (sprite_x + i) % WIDTH;
                        let yj = (sprite_y + j) % HEIGHT;

                        if row & 0x80 >> i != 0 {
                            // Check if any pixel has changed from value 1 to 0
                            self.v[0xF] |= self.pixels[yj % HEIGHT][xi % WIDTH] as u8;
                            self.pixels[yj][xi] ^= true;
                        }
                    }
                }

                // In Extended mode draw pixels at 1:1 scale. Whereas in normal mode upscale
                if self.display_mode == DisplayMode::Extended {
                    display.draw(&self.pixels, 10, 10);
                } else {
                    display.draw(&self.pixels, 20, 20);
                }

                self.draw_flag = true;
                self.pc += 2;
            }

            0xE000 => {
                match self.opcode & 0x00FF {
                    // EX9E Skip next instruction if key stored in Vx is pressed
                    // Usually the next instruction is JMP to skip to a code block
                    0x009E => {
                        if self.keypad[self.v[x] as usize] != 0 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }

                    // EXA1 Skip next instruction if key stored in Vx isn't pressed
                    0x00A1 => {
                        if self.keypad[self.v[x] as usize] != 1 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => println!("Unknown opcode: 0xE000 {:02X}", self.opcode),
                }
            }

            0xF000 => {
                match self.opcode & 0x00FF {
                    // FX07 Set delay timer to Vx
                    0x0007 => {
                        self.v[x] = self.delay_timer;
                        self.pc += 2;
                    }

                    // FX0A Key press awaited then stored in Vx
                    // All instructions halted until next key event
                    // Iterate through all possibilities up to 0xF
                    0x000A => {
                        for i in 0..0xF {
                            if self.keypad[i] != 0 {
                                self.v[x] = i as u8;
                                break;
                            }
                        }
                        self.pc += 2;
                    }

                    // FX15 Set delay timer
                    0x0015 => {
                        self.delay_timer = self.v[x];
                        self.tick = 1.0 / 60.0;
                        self.pc += 2;
                    }

                    // FX18 Set sound timer
                    0x0018 => {
                        self.sound_timer = self.v[x];
                        self.snd_tick = 1.0 / 60.0;
                        self.pc += 2;
                    }

                    // FX1E Add Vx to I (MEM) VF is set to 1 when range overflow (I +VX> 0xFFF)
                    0x001E => {
                        self.i += self.v[x] as u16;
                        self.v[0xF] = (self.i > 0xFFF) as u8;
                        self.pc += 2;
                    }

                    // FX29 Set I to the location of the sprite (5 byte) for char in Vx
                    // Chars 0-F are represented by a 4x5 font Each char contains 5 elements
                    // Create 0x5 font accessible in memory
                    0x0029 => {
                        self.i = (self.v[x] * 0x5) as u16;
                        // if self.mode.debug { println!("At FX29. Vx: {}, I:{}", self.v[x], self.i); }
                        self.pc += 2;
                    }

                    // FX30 SCHIP Set I to the location of the sprite (10 byte) for digit in VX
                    0x0030 => {
                        self.i = (self.v[x] * 10 + 80) as u16;
                        // if self.mode.debug { println!("At FX30. Vx: {}, I:{}", self.v[x], self.i); }

                        self.pc += 2;
                    }

                    // FX33 (BCD) The interpreter takes the decimal value of Vx
                    // & places the hundreds digit in memory at location in I,
                    // the tens digit at location I+1, and the ones digit at location I+2.
                    0x0033 => {
                        let i = self.i as usize;

                        self.memory[i] = self.v[x] / 100;
                        self.memory[i + 1] = self.v[x] % 100 / 10;
                        self.memory[i + 2] = self.v[x] % 10;

                        if self.mode.debug {
                            println!(" BCD: Hundreds:{}, Tens:{}, Ones:{}", self.memory[i], self.memory[i + 1], self.memory[i + 2]);
                        }


                        self.pc += 2;
                    }

                    // FX55 Stores V0 to VX in memory starting at I
                    0x0055 => {

                        for index in 0..(x + 1) {
                            self.memory[index + self.i as usize] = self.v[index];
                        }
                        self.pc += 2;
                    }

                    // FX65 Fills V0 to VX with values from memory starting at I
                    0x0065 => {
                        for index in 0..(x + 1) {
                            self.v[index] = self.memory[(self.i as usize + index)];
                        }
                        self.pc += 2;
                    }

                    // FX75 SCHIP: Store V0 to VX in RPL user flags (X <= 7)
                    0x0075 => {
                        for index in 0..(cmp::max(x as usize, 7) + 1) {
                            self.rpl_flags[index] = self.v[index];
                        }
                        self.pc += 2;
                    }

                    // FX85 SCHIP: Read V0 to VX in RPL user flags (X <= 7)
                    0x0085 => {
                        for index in 0..(cmp::max(x as usize, 7) + 1) {
                            self.v[index] = self.rpl_flags[index];
                        }

                        self.pc += 2;
                    }
                    _ => println!("Unknown opcode: 0x00FF {:X}", self.opcode),
                }
            }
            _ => println!("Unknown opcode: {:X}", self.opcode),
        };
    }

    // Execute fn run() n times
    pub fn step(&mut self, times: u8, mut display: &mut Display) {
        for _ in 0..times {
            self.run(&mut display);
        }
    }
}
