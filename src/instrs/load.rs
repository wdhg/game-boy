use super::instr::{r_from_index, rr_from_index, Instr, Instr::*, Operand::*};
use crate::gameboy::{Reg16::*, Reg8::*};

pub(crate) fn decode(opcode: u8) -> Option<Instr> {
    let reg8_to = (opcode >> 3) & 0b111;
    let reg8_from = opcode & 0b111;
    let reg16_to = (opcode >> 4) & 0b11;

    return match opcode {
        0x0a => Some(LD(R(A), AddressBC)),
        0x1a => Some(LD(R(A), AddressDE)),
        0x02 => Some(LD(AddressBC, R(A))),
        0x12 => Some(LD(AddressDE, R(A))),
        0xfa => Some(LD(R(A), AddressNN)),
        0xea => Some(LD(AddressNN, R(A))),
        0x36 => Some(LD(AddressHL, N)),
        0x22 => Some(LD(AddressHLIncr, R(A))),
        0x2a => Some(LD(R(A), AddressHLIncr)),
        0x32 => Some(LD(AddressHLDecr, R(A))),
        0x3a => Some(LD(R(A), AddressHLDecr)),
        0xe0 => Some(LDH(AddressN, R(A))),
        0xe2 => Some(LDH(AddressC, R(A))),
        0xf0 => Some(LDH(R(A), AddressN)),
        0xf2 => Some(LDH(R(A), AddressC)),
        0x08 => Some(LD(AddressNN, RR(SP))),
        0xf9 => Some(LD(RR(SP), RR(HL))),
        o if o & 0b11111000 == 0b01110000 => Some(LD(AddressHL, r_from_index(reg8_from))),
        o if o & 0b11000111 == 0b01000110 => Some(LD(r_from_index(reg8_to), AddressHL)),
        o if o & 0b11000111 == 0b00000110 => Some(LD(r_from_index(reg8_to), N)),
        o if o & 0b11000000 == 0b01000000 => {
            Some(LD(r_from_index(reg8_to), r_from_index(reg8_from)))
        }
        o if o & 0b11001111 == 0b00000001 => Some(LD(rr_from_index(reg16_to), NN)),
        o if o & 0b11001111 == 0b11000101 => Some(PUSH(rr_from_index(reg16_to))),
        o if o & 0b11001111 == 0b11000001 => Some(POP(rr_from_index(reg16_to))),
        _ => None,
    };
}