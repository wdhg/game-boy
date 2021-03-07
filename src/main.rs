use std::collections::HashMap;

const MEMORY_SIZE: usize = 0x10000;
const INITIAL_PC: u16 = 0x100;
const INITIAL_SP: u16 = 0xfff;

#[derive(PartialEq, Eq, Hash)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(PartialEq, Eq, Hash)]
enum Register16 {
    SP,
    PC,
}

struct GameBoy {
    registers8: HashMap<Register8, u8>,
    registers16: HashMap<Register16, u16>,
    memory: [u8; MEMORY_SIZE], // TODO reduce size of memory to actual GameBoy size instead of the memory map size
}

impl GameBoy {
    fn new() -> GameBoy {
        let mut registers8 = HashMap::new();
        registers8.insert(Register8::A, 0);
        registers8.insert(Register8::B, 0);
        registers8.insert(Register8::C, 0);
        registers8.insert(Register8::D, 0);
        registers8.insert(Register8::E, 0);
        registers8.insert(Register8::F, 0);
        registers8.insert(Register8::H, 0);
        registers8.insert(Register8::L, 0);

        let mut registers16 = HashMap::new();
        registers16.insert(Register16::PC, INITIAL_PC);
        registers16.insert(Register16::SP, INITIAL_SP);

        return GameBoy {
            registers8: registers8,
            registers16: registers16,
            memory: [0; MEMORY_SIZE],
        };
    }
}

fn main() {
    let gb = GameBoy::new();
}
