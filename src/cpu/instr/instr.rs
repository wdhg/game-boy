use crate::cpu::instr::operand::{Op16, Op8};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    NOP,               // no operation
    DAA,               // decimal adjust register A
    CPL,               // complement register A (flip all bits)
    CCF,               // complement carry flag
    SCF,               // set carry flag
    HALT,              // power down CPU until an interrupt occurs
    STOP,              // halt CPU and LCD display until button pressed
    DI,                // disables interrupts
    EI,                // enables interrupts
    LD(Op8, Op8),      // load instruction
    LD16(Op16, Op16),  // 16 bit load instruction
    LDH(Op8, Op8),     // half* load instruction (*half of 16 bit reg)
    PUSH(Op8),         // push instruction
    PUSH16(Op16),      // bit push instruction
    POP(Op8),          // pop instruction
    POP16(Op16),       // 16 bit pop instruction
    ADD(Op8, Op8),     // add instruction
    ADD16(Op16, Op16), // 16 bit add instruction
    ADC(Op8, Op8),     // add with carry instruction
    SUB(Op8),          // sub instruction
    SBC(Op8),          // sub with carry instruction
    AND(Op8),          // and instruction
    OR(Op8),           // or instruction
    XOR(Op8),          // xor instruction
    CP(Op8),           // compare instruction
    INC(Op8),          // increment instruction
    INC16(Op16),       // 16 bit increment instruction
    DEC(Op8),          // decrement instruction
    DEC16(Op16),       // 16 bit decrement instruction
    RLC(Op8),          // rotate left with carry
    RL(Op8),           // rotate left
    RRC(Op8),          // rotate right with carry
    RR(Op8),           // rotate right
    SLA(Op8),          // shift left into carry (LSB = 0)
    SRA(Op8),          // shift right into carry (MSB constant)
    SRL(Op8),          // shift left into carry (MSB = 0)
    BIT(u8, Op8),      // test bit in register
    SET(u8, Op8),      // set bit in register
    RES(u8, Op8),      // reset bit in register
}
