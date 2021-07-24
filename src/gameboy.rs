use crate::instrs::instr::Instr;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Reg16 {
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Reg16 {
    pub fn from_index(i: u8) -> Reg16 {
        use Reg16::*;
        // i is 2 bits long
        match i {
            0b00 => BC,
            0b01 => DE,
            0b10 => HL,
            0b11 => SP,
            _ => panic!("Illegal 16-bit register index {}", i),
        }
    }
}

enum CPUState {
    Fetch,
    FetchPrefixed,
    Excute(Instr, u64), // instruction and start cycle
}

pub struct GameBoy {
    pub memory: [u8; MEMORY_SIZE],
    registers8: HashMap<Reg8, u8>,
    registers16: HashMap<Reg16, u16>,
    state: CPUState,
    cycle: u64, // machine cycles ()
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
                Reg8::F => 0, // flag register with bits: ZNHC0000
                Reg8::H => 0,
                Reg8::L => 0,
            },
            registers16: hashmap! {
                Reg16::PC => INITIAL_PC,
                Reg16::SP => INITIAL_SP,
            },
            memory: [0; MEMORY_SIZE],
            state: CPUState::Fetch,
            cycle: 0,
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

    fn read_next_opcode(&mut self) -> u8 {
        let pc = self.read_register16(Reg16::PC);
        let opcode: u8 = self.memory[pc as usize];
        self.write_register16(Reg16::PC, pc + 1);
        self.increment_cycle();
        return opcode;
    }

    fn increment_cycle(&mut self) {
        self.cycle += 1;
    }

    // a single machine cycle (4 clock cycles)
    pub fn cycle(&mut self) {
        use CPUState::*;

        match self.state {
            Fetch => self.fetch(),
            FetchPrefixed => self.fetch_prefixed(),
            Excute(instr, start_cycle) => self.execute(instr, start_cycle),
        }

        /*
         * TODO if doing last execution cycle, pre-fetch next opcode:
         * - maybe don't call increment_cycles to emulate this? this would violate the contract that
         *   this function is a single machine cycle
         * - make a call to fetch()? probably more accurate to what is actually happening in
         *   hardware
         */
        self.increment_cycle();
    }

    fn fetch(&mut self) {
        use crate::instrs::{decode_unprefixed, PREFIX};
        let opcode: u8 = self.read_next_opcode();

        if opcode == PREFIX {
            self.state = CPUState::FetchPrefixed;
        } else {
            self.state = CPUState::Excute(decode_unprefixed(opcode), self.cycle);
        }
    }

    fn fetch_prefixed(&mut self) {
        use crate::instrs::decode_prefixed;
        let opcode: u8 = self.read_next_opcode();
        self.state = CPUState::Excute(decode_prefixed(opcode), self.cycle);
    }

    fn execute(&mut self, instr: Instr, start_cycle: u64) {
        use crate::instrs::instr::Instr::*;

        let cycle: u8 = (self.cycle - start_cycle) as u8;

        // functions defined in cpu_funcs.rs
        match instr {
            NOP => self.nop(cycle),
            DAA => self.daa(cycle),
            CPL => self.cpl(cycle),
            CCF => self.ccf(cycle),
            SCF => self.scf(cycle),
            HALT => self.halt(cycle),
            STOP => self.stop(cycle),
            DI => self.di(cycle),
            EI => self.ei(cycle),
            LD(op1, op2) => self.ld(cycle, op1, op2),
            LDH(op1, op2) => self.ldh(cycle, op1, op2),
            PUSH(op) => self.push(cycle, op),
            POP(op) => self.pop(cycle, op),
            ADD(op1, op2) => self.add(cycle, op1, op2),
            ADC(op1, op2) => self.adc(cycle, op1, op2),
            SUB(op) => self.sub(cycle, op),
            SBC(op) => self.sbc(cycle, op),
            AND(op) => self.and(cycle, op),
            OR(op) => self.or(cycle, op),
            XOR(op) => self.xor(cycle, op),
            CP(op) => self.cp(cycle, op),
            INC(op) => self.inc(cycle, op),
            DEC(op) => self.dec(cycle, op),
            RLC(op) => self.rlc(cycle, op),
            RL(op) => self.rl(cycle, op),
            RRC(op) => self.rrc(cycle, op),
            RR(op) => self.rr(cycle, op),
            SLA(op) => self.sla(cycle, op),
            SRA(op) => self.sra(cycle, op),
            SRL(op) => self.srl(cycle, op),
            BIT(op1, op2) => self.bit(cycle, op1, op2),
            SET(op1, op2) => self.set(cycle, op1, op2),
            RES(op1, op2) => self.res(cycle, op1, op2),
        }
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
