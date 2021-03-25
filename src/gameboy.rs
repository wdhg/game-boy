use std::collections::HashMap;

extern crate maplit;

const MEMORY_SIZE: usize = 0x10000;
const INITIAL_PC: u16 = 0x100;
const INITIAL_SP: u16 = 0xfff;

#[derive(PartialEq, Eq, Hash)]
enum Reg8 {
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
enum Reg16 {
    SP,
    PC,
}

pub struct GameBoy {
    pub memory: [u8; MEMORY_SIZE],
    registers8: HashMap<Reg8, u8>,
    registers16: HashMap<Reg16, u16>,
}

impl GameBoy {
    pub fn new() -> GameBoy {
        return GameBoy {
            registers8: maplit::hashmap! {
                Reg8::A => 0,
                Reg8::B => 0,
                Reg8::C => 0,
                Reg8::D => 0,
                Reg8::E => 0,
                Reg8::F => 0,
                Reg8::H => 0,
                Reg8::L => 0,
            },
            registers16: maplit::hashmap! {
                Reg16::PC => INITIAL_PC,
                Reg16::SP => INITIAL_SP,
            },
            memory: [0; MEMORY_SIZE],
        };
    }

    pub fn read_register8(&self, r: Reg8) -> u8 {
        return self.registers8.get(&r).copied().unwrap_or_default();
    }

    pub fn write_register8(&mut self, r: Reg8, v: u8) {
        self.registers8.entry(r).and_modify(|e| *e = v).or_default();
    }
}
