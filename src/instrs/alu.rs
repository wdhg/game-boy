use super::instr::{operand16_from_index, operand8_from_index, Instr, Instr::*, Operand::*};
use crate::gameboy::Reg16::*;
use crate::gameboy::Reg8::*;

pub(crate) fn decode_unprefixed(opcode: u8) -> Option<Instr> {
    let op8_to = operand8_from_index((opcode >> 3) & 0b111);
    let op8_from = operand8_from_index(opcode & 0b111);
    let op16_from = operand16_from_index((opcode >> 4) & 0b11);

    return match opcode {
        0xc6 => Some(ADD(R8(A), N)),
        0xce => Some(ADC(R8(A), N)),
        0xd6 => Some(SUB(N)),
        0xe6 => Some(AND(N)),
        0xf6 => Some(OR(N)),
        0xee => Some(XOR(N)),
        0xfe => Some(CP(N)),
        0xe8 => Some(ADD(R16(SP), N)),
        0x07 => Some(RLC(R8(A))),
        0x17 => Some(RL(R8(A))),
        0x0f => Some(RRC(R8(A))),
        0x1f => Some(RR(R8(A))),
        o if o & 0b11111000 == 0x80 => Some(ADD(R8(A), op8_from)),
        o if o & 0b11111000 == 0x88 => Some(ADC(R8(A), op8_from)),
        o if o & 0b11111000 == 0x90 => Some(SUB(op8_from)),
        o if o & 0b11111000 == 0x98 => Some(SBC(op8_from)),
        o if o & 0b11111000 == 0xa0 => Some(AND(op8_from)),
        o if o & 0b11111000 == 0xb0 => Some(OR(op8_from)),
        o if o & 0b11111000 == 0xa8 => Some(XOR(op8_from)),
        o if o & 0b11111000 == 0xb8 => Some(CP(op8_from)),
        o if o & 0b11000111 == 0x04 => Some(INC(op8_to)),
        o if o & 0b11000111 == 0x05 => Some(DEC(op8_to)),
        o if o & 0b11001111 == 0x09 => Some(ADD(R16(HL), op16_from)),
        o if o & 0b11001111 == 0x03 => Some(INC(op16_from)),
        o if o & 0b11001111 == 0x0b => Some(DEC(op16_from)),
        _ => None,
    };
}

pub(crate) fn decode_prefixed(opcode: u8) -> Option<Instr> {
    let op8 = operand8_from_index(opcode & 0b111);

    return match opcode {
        o if o & 0b11111000 == 0x00 => Some(RLC(op8)),
        o if o & 0b11111000 == 0x10 => Some(RL(op8)),
        o if o & 0b11111000 == 0x08 => Some(RRC(op8)),
        o if o & 0b11111000 == 0x18 => Some(RR(op8)),
        o if o & 0b11111000 == 0x20 => Some(SLA(op8)),
        o if o & 0b11111000 == 0x28 => Some(SRA(op8)),
        o if o & 0b11111000 == 0x38 => Some(SRL(op8)),
        _ => None,
    };
}
