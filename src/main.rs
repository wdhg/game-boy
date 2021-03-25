use std::collections::HashMap;

extern crate maplit;

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
    memory: [u8; MEMORY_SIZE],
}

impl GameBoy {
    fn new() -> GameBoy {
        return GameBoy {
            registers8: maplit::hashmap! {
                Register8::A => 0,
                Register8::B => 0,
                Register8::C => 0,
                Register8::D => 0,
                Register8::E => 0,
                Register8::F => 0,
                Register8::H => 0,
                Register8::L => 0,
            },
            registers16: maplit::hashmap! {
                Register16::PC => INITIAL_PC,
                Register16::SP => INITIAL_SP,
            },
            memory: [0; MEMORY_SIZE],
        };
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
