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
        let mut gb = GameBoy {
            registers8: HashMap::new(),
            registers16: HashMap::new(),
            memory: [0; MEMORY_SIZE],
        };

        gb.registers8.insert(Register8::A, 0);
        gb.registers8.insert(Register8::B, 0);
        gb.registers8.insert(Register8::C, 0);
        gb.registers8.insert(Register8::D, 0);
        gb.registers8.insert(Register8::E, 0);
        gb.registers8.insert(Register8::F, 0);
        gb.registers8.insert(Register8::H, 0);
        gb.registers8.insert(Register8::L, 0);

        gb.registers16.insert(Register16::PC, INITIAL_PC);
        gb.registers16.insert(Register16::SP, INITIAL_SP);

        return gb;
    }

    fn read_register8(&self, r: Register8) -> u8 {
        return self.registers8.get(&r).copied().unwrap_or_default();
    }

    fn write_register8(&mut self, r: Register8, v: u8) {
        self.registers8.entry(r).and_modify(|e| *e = v).or_default();
    }
}

fn main() {
    let gb = &mut GameBoy::new();
}
