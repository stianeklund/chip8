#[derive(Debug)
pub struct Cpu {
    opcode: u16,
    memory: [u8, 4096], // should perhaps be 4095?
    v: [u8; 16],
    i: isize,
    pc: usize,
    stack: [u16; 16],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
}

  impl Cpu {
      pub fn new() -> Cpu {
          let mut cpu = Cpu {
              opcode: 0,
              memory: [0; 4096],
              v: [0; 16],
              i: 0x200, // start address for programs
              pc: 0x200, // PC reset goes to same address
              stack: [0; 16],
              sp: 0,
              delay_timer: 0,
              sound_timer: 0
          }
      }
  }


