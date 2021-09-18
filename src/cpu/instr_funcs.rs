use crate::cpu::instr::operand::{Op16, Op8};
use crate::cpu::{CPUState, Flag, Reg8, CPU};

impl CPU {
    pub fn nop(&mut self) {
        return;
    }

    // correcting calculation result between two BCD numbers back to BCD
    // https://ehaskins.com/2018-01-30%20Z80%20DAA/
    // if adding: add 6 to each digit greater than 9, or if it carried
    // if subtracting: subtract 6 from each digit greater than 9, or if it carried
    pub fn daa(&mut self) {
        let value: u8 = self.read_reg8(Reg8::A);
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

        self.write_reg8(Reg8::A, new_value);
    }

    pub fn cpl(&mut self) {
        self.write_reg8(Reg8::A, !self.read_reg8(Reg8::A));
    }

    pub fn ccf(&mut self) {
        self.set_flag_to(Flag::N, false);
        self.set_flag_to(Flag::H, false);
        self.set_flag_to(Flag::C, !self.test_flag(Flag::C));
    }

    pub fn scf(&mut self) {
        self.set_flag_to(Flag::N, false);
        self.set_flag_to(Flag::H, false);
        self.set_flag_to(Flag::C, true);
    }

    pub fn halt(&mut self) {
        self.state = CPUState::Halted;
    }

    pub fn stop(&mut self) {
        self.state = CPUState::Stopped;
    }

    pub fn di(&mut self) {
        self.interrupt_master_enable = false;
    }

    pub fn ei(&mut self) {
        self.interrupt_master_enable = true;
    }

    pub fn ld(&mut self, cycle: u8, op1: Op8, op2: Op8) {}

    pub fn ld16(&mut self, cycle: u8, op1: Op16, op2: Op16) {}

    pub fn ldh(&mut self, cycle: u8, op1: Op8, op2: Op8) {}

    pub fn push(&mut self, cycle: u8, op: Op16) {}

    pub fn pop(&mut self, cycle: u8, op: Op16) {}

    pub fn add(&mut self, cycle: u8, op1: Op8, op2: Op8) {}

    pub fn add16(&mut self, cycle: u8, op1: Op16, op2: Op16) {}

    pub fn adc(&mut self, cycle: u8, op1: Op8, op2: Op8) {}

    pub fn sub(&mut self, cycle: u8, op: Op8) {}

    pub fn sbc(&mut self, cycle: u8, op: Op8) {}

    pub fn and(&mut self, cycle: u8, op: Op8) {}

    pub fn or(&mut self, cycle: u8, op: Op8) {}

    pub fn xor(&mut self, cycle: u8, op: Op8) {}

    pub fn cp(&mut self, cycle: u8, op: Op8) {}

    pub fn inc(&mut self, cycle: u8, op: Op8) {}

    pub fn inc16(&mut self, cycle: u8, op: Op16) {}

    pub fn dec(&mut self, cycle: u8, op: Op8) {}

    pub fn dec16(&mut self, cycle: u8, op: Op16) {}

    pub fn rlc(&mut self, cycle: u8, op: Op8) {}

    pub fn rl(&mut self, cycle: u8, op: Op8) {}

    pub fn rrc(&mut self, cycle: u8, op: Op8) {}

    pub fn rr(&mut self, cycle: u8, op: Op8) {}

    pub fn sla(&mut self, cycle: u8, op: Op8) {}

    pub fn sra(&mut self, cycle: u8, op: Op8) {}

    pub fn srl(&mut self, cycle: u8, op: Op8) {}

    pub fn bit(&mut self, cycle: u8, index: u8, op: Op8) {}

    pub fn set(&mut self, cycle: u8, index: u8, op: Op8) {}

    pub fn res(&mut self, cycle: u8, index: u8, op: Op8) {}
}
