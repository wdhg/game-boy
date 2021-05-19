use super::instr::{operand16_from_index, operand8_from_index, Instr, Instr::*, Operand::*};
use crate::gameboy::{Reg16::*, Reg8::*};

pub(crate) fn decode(opcode: u8) -> Option<Instr> {
    let op8_to = operand8_from_index((opcode >> 3) & 0b111);
    let op8_from = operand8_from_index(opcode & 0b111);
    let op16_to = operand16_from_index((opcode >> 4) & 0b11);

    return match opcode {
        0x0a => Some(LD(R8(A), AddressBC)),
        0x1a => Some(LD(R8(A), AddressDE)),
        0x02 => Some(LD(AddressBC, R8(A))),
        0x12 => Some(LD(AddressDE, R8(A))),
        0xfa => Some(LD(R8(A), AddressNN)),
        0xea => Some(LD(AddressNN, R8(A))),
        0x36 => Some(LD(AddressHL, N)),
        0x22 => Some(LD(AddressHLIncr, R8(A))),
        0x2a => Some(LD(R8(A), AddressHLIncr)),
        0x32 => Some(LD(AddressHLDecr, R8(A))),
        0x3a => Some(LD(R8(A), AddressHLDecr)),
        0xe0 => Some(LDH(AddressN, R8(A))),
        0xe2 => Some(LDH(AddressC, R8(A))),
        0xf0 => Some(LDH(R8(A), AddressN)),
        0xf2 => Some(LDH(R8(A), AddressC)),
        0x08 => Some(LD(AddressNN, R16(SP))),
        0xf9 => Some(LD(R16(SP), R16(HL))),
        o if o & 0b11111000 == 0x70 => Some(LD(AddressHL, op8_from)),
        o if o & 0b11000111 == 0x45 => Some(LD(op8_to, AddressHL)),
        o if o & 0b11000111 == 0x06 => Some(LD(op8_to, N)),
        o if o & 0b11000000 == 0x40 => Some(LD(op8_to, op8_from)),
        o if o & 0b11001111 == 0x01 => Some(LD(op16_to, NN)),
        o if o & 0b11001111 == 0xc5 => Some(PUSH(op16_to)),
        o if o & 0b11001111 == 0xc1 => Some(POP(op16_to)),
        _ => None,
    };
}
