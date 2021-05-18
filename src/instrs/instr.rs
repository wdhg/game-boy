use crate::gameboy::{Reg16, Reg8};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    R8(Reg8),      // 8 bit register
    N,             // 8 bit number following opcode
    AddressN,      // 8 bit address following opcode
    AddressC,      // 8 bit address stored in C
    R16(Reg16),    // 16 bit register
    NN,            // 16 bit number following opcode
    AddressNN,     // 16 bit address following opcode
    AddressBC,     // 16 bit address stored in B and C
    AddressDE,     // 16 bit address stored in D and E
    AddressHL,     // 16 bit address stored in H and L
    AddressHLIncr, // 16 bit address stored in H and L, incremented after use
    AddressHLDecr, // 16 bit address stored in H and L, decremented after use
}

pub fn r_from_index(i: u8) -> Operand {
    return Operand::R8(Reg8::from_index(i));
}

pub fn rr_from_index(i: u8) -> Operand {
    return Operand::R16(Reg16::from_index(i));
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    NOP,                   // no operation
    DAA,                   // decimal adjust register A
    CPL,                   // complement register A (flip all bits)
    CCF,                   // complement carry flag
    SCF,                   // set carry flag
    HALT,                  // power down CPU until an interrupt occurs
    STOP,                  // halt CPU and LCD display until button pressed
    DI,                    // disables interrupts
    EI,                    // enables interrupts
    LD(Operand, Operand),  // load instruction
    LDH(Operand, Operand), // load instruction
    PUSH(Operand),         // push instruction
    POP(Operand),          // pop instruction
    ADD(Operand, Operand), // add instruction
    ADC(Operand, Operand), // add with carry instruction
    SUB(Operand),          // sub instruction
    SBC(Operand),          // sub with carry instruction
    AND(Operand),          // and instruction
    OR(Operand),           // or instruction
    XOR(Operand),          // xor instruction
    CP(Operand),           // compare instruction
    INC(Operand),          // increment instruction
    DEC(Operand),          // decrement instruction
    RLC(Operand),          // rotate left with carry
    RL(Operand),           // rotate left
    RRC(Operand),          // rotate right with carry
    RR(Operand),           // rotate right
}
