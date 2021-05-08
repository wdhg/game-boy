use super::instr::{r_from_index, Instr, Instr::*, Operand::*};
use crate::gameboy::Reg8::*;

pub(crate) fn decode(opcode: u8) -> Option<Instr> {
    let reg8_to = (opcode >> 3) & 0b111;
    let reg8_from = opcode & 0b111;

    return match opcode {
        0xc6 => Some(ADD(R(A), N)),
        0x86 => Some(ADD(R(A), AddressHL)),
        0xce => Some(ADC(R(A), N)),
        0x8e => Some(ADC(R(A), AddressHL)),
        0x96 => Some(SUB(AddressHL)),
        0xd6 => Some(SUB(N)),
        0x9e => Some(SBC(AddressHL)),
        0xa6 => Some(AND(AddressHL)),
        0xe6 => Some(AND(N)),
        0xb6 => Some(OR(AddressHL)),
        0xf6 => Some(OR(N)),
        0xae => Some(XOR(AddressHL)),
        0xee => Some(XOR(N)),
        0xbe => Some(CP(AddressHL)),
        0xfe => Some(CP(N)),
        0x34 => Some(INC(AddressHL)),
        0x35 => Some(DEC(AddressHL)),
        o if o & 0b11111000 == 0b10000000 => Some(ADD(R(A), r_from_index(reg8_from))),
        o if o & 0b11111000 == 0b10001000 => Some(ADC(R(A), r_from_index(reg8_from))),
        o if o & 0b11111000 == 0b10010000 => Some(SUB(r_from_index(reg8_from))),
        o if o & 0b11111000 == 0b10011000 => Some(SBC(r_from_index(reg8_from))),
        o if o & 0b11111000 == 0b10100000 => Some(AND(r_from_index(reg8_from))),
        o if o & 0b11111000 == 0b10110000 => Some(OR(r_from_index(reg8_from))),
        o if o & 0b11111000 == 0b10101000 => Some(XOR(r_from_index(reg8_from))),
        o if o & 0b11111000 == 0b10111000 => Some(CP(r_from_index(reg8_from))),
        o if o & 0b11000111 == 0b00000100 => Some(INC(r_from_index(reg8_to))),
        o if o & 0b11000111 == 0b00000101 => Some(DEC(r_from_index(reg8_to))),
        _ => None,
    };
}