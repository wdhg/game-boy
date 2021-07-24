use crate::gameboy::{Flag, GameBoy, Reg8};
use crate::instrs::instr::Operand;

impl GameBoy {
    pub fn nop(&mut self, cycle: u8) {
        return;
    }

    // correcting calculation result between two BCD numbers back to BCD
    // https://ehaskins.com/2018-01-30%20Z80%20DAA/
    // if adding: add 6 to each digit greater than 9, or if it carried
    // if subtracting: subtract 6 from each digit greater than 9, or if it carried
    pub fn daa(&mut self, cycle: u8) {
        let value: u8 = self.read_register8(Reg8::A);
        let lower_digit: u8 = value & 0xf;
        let upper_digit: u8 = (value >> 4) & 0xf;

        let mut adjustment: u8 = 0x00;

        // flags
        let subtraction: bool = self.test_flag(Flag::N);
        let carry: bool = self.test_flag(Flag::C);
        let half_carry: bool = self.test_flag(Flag::H);

        // adjustment for least significant digit
        if half_carry || (!subtraction && lower_digit > 9) {
            adjustment += 0x06;
        }

        // adjustment for most significant digit
        if carry || (!subtraction && upper_digit > 9) {
            adjustment += 0x60;
        }

        // apply adjustment
        let new_value = if subtraction {
            value - adjustment
        } else {
            value + adjustment
        };

        self.write_register8(Reg8::A, new_value);
    }

    pub fn cpl(&mut self, cycle: u8) {}

    pub fn ccf(&mut self, cycle: u8) {}

    pub fn scf(&mut self, cycle: u8) {}

    pub fn halt(&mut self, cycle: u8) {}

    pub fn stop(&mut self, cycle: u8) {}

    pub fn di(&mut self, cycle: u8) {}

    pub fn ei(&mut self, cycle: u8) {}

    pub fn ld(&mut self, cycle: u8, op1: Operand, op2: Operand) {}

    pub fn ldh(&mut self, cycle: u8, op1: Operand, op2: Operand) {}

    pub fn push(&mut self, cycle: u8, op: Operand) {}

    pub fn pop(&mut self, cycle: u8, op: Operand) {}

    pub fn add(&mut self, cycle: u8, op1: Operand, op2: Operand) {}

    pub fn adc(&mut self, cycle: u8, op1: Operand, op2: Operand) {}

    pub fn sub(&mut self, cycle: u8, op: Operand) {}

    pub fn sbc(&mut self, cycle: u8, op: Operand) {}

    pub fn and(&mut self, cycle: u8, op: Operand) {}

    pub fn or(&mut self, cycle: u8, op: Operand) {}

    pub fn xor(&mut self, cycle: u8, op: Operand) {}

    pub fn cp(&mut self, cycle: u8, op: Operand) {}

    pub fn inc(&mut self, cycle: u8, op: Operand) {}

    pub fn dec(&mut self, cycle: u8, op: Operand) {}

    pub fn rlc(&mut self, cycle: u8, op: Operand) {}

    pub fn rl(&mut self, cycle: u8, op: Operand) {}

    pub fn rrc(&mut self, cycle: u8, op: Operand) {}

    pub fn rr(&mut self, cycle: u8, op: Operand) {}

    pub fn sla(&mut self, cycle: u8, op: Operand) {}

    pub fn sra(&mut self, cycle: u8, op: Operand) {}

    pub fn srl(&mut self, cycle: u8, op: Operand) {}

    pub fn bit(&mut self, cycle: u8, op1: Operand, op2: Operand) {}

    pub fn set(&mut self, cycle: u8, op1: Operand, op2: Operand) {}

    pub fn res(&mut self, cycle: u8, op1: Operand, op2: Operand) {}
}
