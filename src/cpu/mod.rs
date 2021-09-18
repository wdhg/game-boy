mod instr;
mod instr_funcs;

extern crate maplit;
use crate::cpu::instr::instr::Instr;
use crate::gameboy::MEMORY_SIZE;

const INITIAL_PC: u16 = 0x100;
const INITIAL_SP: u16 = 0xfff;
const REG_8_COUNT: usize = 8;
const REG_16_COUNT: usize = 2; // SP and PC only

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
    pub(crate) fn from_index(i: u8) -> Reg8 {
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
    SP,
    PC,
    BC,
    DE,
    HL,
}

impl Reg16 {
    pub(crate) fn from_index(i: u8) -> Reg16 {
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

    pub(crate) fn t() {}
}

// flag bits for the flag register F
pub(crate) enum Flag {
    Z, // zero flag: result = 0
    N, // subtract flag: subtraction was performed in last instruction
    H, // half carry flag: carry from lower nibble occurred
    C, // carry flag: carry occurred or if reg A is the smaller value when executing CP
}

impl Flag {
    pub fn bit(&self) -> u8 {
        use Flag::*;

        match self {
            Z => 7,
            N => 6,
            H => 5,
            C => 4,
        }
    }
}

pub enum CPUState {
    Fetch,
    FetchPrefixed,
    Excute(Instr, u128), // instruction and start cycle
    Halted,
    Stopped,
}

pub struct CPU {
    registers8: [u8; REG_8_COUNT],
    registers16: [u16; REG_16_COUNT],
    pub state: CPUState,
    pub interrupt_master_enable: bool,
    cycle: u128, // machine cycles
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            registers8: [0; REG_8_COUNT],
            registers16: [INITIAL_SP, INITIAL_PC],
            state: CPUState::Fetch,
            interrupt_master_enable: true,
            cycle: 0,
        };
    }

    pub fn increment_cycle(&mut self) {
        self.cycle += 1;
    }

    pub(crate) fn read_reg8(&self, r: Reg8) -> u8 {
        return self.registers8[r as usize];
    }

    pub(crate) fn write_reg8(&mut self, r: Reg8, value: u8) {
        self.registers8[r as usize] = value;
    }

    pub(crate) fn read_reg16(&self, r: Reg16) -> u16 {
        return self.registers16[r as usize];
    }

    pub(crate) fn write_reg16(&mut self, r: Reg16, value: u16) {
        self.registers16[r as usize] = value;
    }

    pub(crate) fn set_flag_to(&mut self, flag: Flag, value: bool) {
        let flag_bits = self.read_reg8(Reg8::F);
        let mask = 1 << flag.bit();
        let new_flag_bits = if value {
            flag_bits | mask // setting
        } else {
            flag_bits & !mask // resetting
        };
        self.write_reg8(Reg8::F, new_flag_bits);
    }

    pub(crate) fn test_flag(&self, flag: Flag) -> bool {
        let index = flag.bit();
        return (self.read_reg8(Reg8::F) >> index) & 1 == 1;
    }

    pub fn pc_read_next(&mut self, memory: &[u8; MEMORY_SIZE]) -> u8 {
        let pc: u16 = self.read_reg16(Reg16::PC);
        let byte: u8 = memory[pc as usize];
        self.write_reg16(Reg16::PC, pc + 1);
        return byte;
    }

    fn read_next_opcode(&mut self, memory: &[u8; MEMORY_SIZE]) -> u8 {
        let pc = self.read_reg16(Reg16::PC);
        let opcode: u8 = memory[pc as usize];
        self.write_reg16(Reg16::PC, pc + 1);
        self.increment_cycle();
        return opcode;
    }

    // a single machine cycle (4 clock cycles)
    pub fn cycle(&mut self, memory: &[u8; MEMORY_SIZE]) {
        use crate::cpu::CPUState::*;

        match self.state {
            Fetch => self.fetch(memory),
            FetchPrefixed => self.fetch_prefixed(memory),
            Excute(instr, start_cycle) => self.execute(instr, start_cycle),
            Halted => return,
            Stopped => return, // TODO continue on button press
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

    fn fetch(&mut self, memory: &[u8; MEMORY_SIZE]) {
        use crate::cpu::instr::{decode_unprefixed, PREFIX};
        let opcode: u8 = self.read_next_opcode(memory);

        if opcode == PREFIX {
            self.state = CPUState::FetchPrefixed;
        } else {
            self.state = CPUState::Excute(decode_unprefixed(opcode), self.cycle);
        }
    }

    fn fetch_prefixed(&mut self, memory: &[u8; MEMORY_SIZE]) {
        use crate::cpu::instr::decode_prefixed;
        let opcode: u8 = self.read_next_opcode(memory);
        self.state = CPUState::Excute(decode_prefixed(opcode), self.cycle);
    }

    fn execute(&mut self, instr: Instr, start_cycle: u128) {
        use crate::cpu::instr::instr::Instr::*;

        let cycle: u8 = (self.cycle - start_cycle) as u8;

        // functions defined in cpu_funcs.rs
        match instr {
            NOP => self.nop(),
            DAA => self.daa(),
            CPL => self.cpl(),
            CCF => self.ccf(),
            SCF => self.scf(),
            HALT => self.halt(),
            STOP => self.stop(),
            DI => self.di(),
            EI => self.ei(),
            LD(op1, op2) => self.ld(cycle, op1, op2),
            LD16(op1, op2) => self.ld16(cycle, op1, op2),
            LDH(op1, op2) => self.ldh(cycle, op1, op2),
            PUSH(op) => self.push(cycle, op),
            POP(op) => self.pop(cycle, op),
            ADD(op1, op2) => self.add(cycle, op1, op2),
            ADD16(op1, op2) => self.add16(cycle, op1, op2),
            ADC(op1, op2) => self.adc(cycle, op1, op2),
            SUB(op) => self.sub(cycle, op),
            SBC(op) => self.sbc(cycle, op),
            AND(op) => self.and(cycle, op),
            OR(op) => self.or(cycle, op),
            XOR(op) => self.xor(cycle, op),
            CP(op) => self.cp(cycle, op),
            INC(op) => self.inc(cycle, op),
            INC16(op) => self.inc16(cycle, op),
            DEC(op) => self.dec(cycle, op),
            DEC16(op) => self.dec16(cycle, op),
            RLC(op) => self.rlc(cycle, op),
            RL(op) => self.rl(cycle, op),
            RRC(op) => self.rrc(cycle, op),
            RR(op) => self.rr(cycle, op),
            SLA(op) => self.sla(cycle, op),
            SRA(op) => self.sra(cycle, op),
            SRL(op) => self.srl(cycle, op),
            BIT(index, op) => self.bit(cycle, index, op),
            SET(index, op) => self.set(cycle, index, op),
            RES(index, op) => self.res(cycle, index, op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_write_8_bit_reg8() {
        use Reg8::*;

        let mut cpu = CPU::new();

        cpu.write_reg8(A, 0x01);
        cpu.write_reg8(F, 0x23);
        cpu.write_reg8(L, 0x45);

        assert_eq!(cpu.read_reg8(A), 0x01);
        assert_eq!(cpu.read_reg8(B), 0x00);
        assert_eq!(cpu.read_reg8(C), 0x00);
        assert_eq!(cpu.read_reg8(D), 0x00);
        assert_eq!(cpu.read_reg8(E), 0x00);
        assert_eq!(cpu.read_reg8(F), 0x23);
        assert_eq!(cpu.read_reg8(H), 0x00);
        assert_eq!(cpu.read_reg8(L), 0x45);
    }

    #[test]
    fn test_read_and_write_16_bit_regs() {
        use Reg16::*;

        let mut cpu = CPU::new();

        cpu.write_reg16(PC, 0xabcd);
        cpu.write_reg16(SP, 0xef01);

        assert_eq!(cpu.read_reg16(PC), 0xabcd);
        assert_eq!(cpu.read_reg16(SP), 0xef01);
    }
}
