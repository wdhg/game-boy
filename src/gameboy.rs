use std::collections::HashMap;

extern crate maplit;
use maplit::hashmap;

const MEMORY_SIZE: usize = 0x10000;
const INITIAL_PC: u16 = 0x100;
const INITIAL_SP: u16 = 0xfff;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

impl Reg8 {
    pub fn from_index(i: u8) -> Reg8 {
        use Reg8::*;
        // i is 3 bits long
        match i {
            0b000 => B, // 0
            0b001 => C, // 1
            0b010 => D, // 2
            0b011 => E, // 3
            0b100 => H, // 4
            0b101 => L, // 5
            0b111 => A, // 7
            _ => panic!("Illegal 8-bit register index {}", i),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg16 {
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
            registers8: hashmap! {
                Reg8::A => 0,
                Reg8::B => 0,
                Reg8::C => 0,
                Reg8::D => 0,
                Reg8::E => 0,
                Reg8::F => 0,
                Reg8::H => 0,
                Reg8::L => 0,
            },
            registers16: hashmap! {
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

    pub fn read_register16(&self, r: Reg16) -> u16 {
        return self.registers16.get(&r).copied().unwrap_or_default();
    }

    pub fn write_register16(&mut self, r: Reg16, v: u16) {
        self.registers16
            .entry(r)
            .and_modify(|e| *e = v)
            .or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_write_8_bit_registers() {
        use Reg8::*;

        let gb = &mut GameBoy::new();

        gb.write_register8(A, 0x01);
        gb.write_register8(F, 0x23);
        gb.write_register8(L, 0x45);

        assert_eq!(gb.read_register8(A), 0x01);
        assert_eq!(gb.read_register8(B), 0x00);
        assert_eq!(gb.read_register8(C), 0x00);
        assert_eq!(gb.read_register8(D), 0x00);
        assert_eq!(gb.read_register8(E), 0x00);
        assert_eq!(gb.read_register8(F), 0x23);
        assert_eq!(gb.read_register8(H), 0x00);
        assert_eq!(gb.read_register8(L), 0x45);
    }

    #[test]
    fn test_read_and_write_16_bit_registers() {
        use Reg16::*;

        let gb = &mut GameBoy::new();

        gb.write_register16(PC, 0xabcd);
        gb.write_register16(SP, 0xef01);

        assert_eq!(gb.read_register16(PC), 0xabcd);
        assert_eq!(gb.read_register16(SP), 0xef01);
    }
}
