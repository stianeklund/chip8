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
    // keypad: Keypad,
    // display: Display
}

// TODO: Write keypad & display implementation in SDL2
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            opcode: 0,
            memory: [0; 4096],  // 0x000 (0) to 0xFFF (4095). 0x000 - 0x1FF for interpreter
            v: [0; 16],         // 8-bit general purpose register, ()V0 - VE*).
            i: 0x200,           // Index register (start address)
            pc: 0x200,          // Program Counter (start address is 0x200 on RST)
            stack: [0; 16],     // Interpreter returns to value when done with subroutine
            sp: 0,              // Stack pointer. Used to point to topmost level of the Stack
            dt: 0,              // 8-bit Delay Timer
            st: 0,              // 8-bit Sound timer
            // * VF is a special register used to store the overflow bit
        }
    }
}
// TODO: Read instructions from memory & execute instructions + read up on opcodes.
// Run match statement over opcode for stepping?
