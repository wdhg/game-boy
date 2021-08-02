use crate::cpu::CPU;

pub const MEMORY_SIZE: usize = 0x10000;

pub struct GameBoy {
    pub memory: [u8; MEMORY_SIZE],
    pub cpu: CPU,
}

impl GameBoy {
    pub fn new() -> GameBoy {
        return GameBoy {
            memory: [0; MEMORY_SIZE],
            cpu: CPU::new(),
        };
    }
}
