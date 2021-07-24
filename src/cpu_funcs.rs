use crate::gameboy::GameBoy;
use crate::instrs::instr::Operand;

impl GameBoy {
    pub fn nop(&mut self, cycle: u8) {}

    pub fn daa(&mut self, cycle: u8) {}

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
