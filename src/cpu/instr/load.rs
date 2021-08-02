use crate::cpu::instr::operand::{operand16_from_index, operand8_from_index, Op16, Op8};
use crate::cpu::instr::{Instr, Instr::*};
use crate::cpu::{Reg16::*, Reg8::*};

pub(crate) fn decode_unprefixed(opcode: u8) -> Option<Instr> {
    let op8_to = operand8_from_index((opcode >> 3) & 0b111);
    let op8_from = operand8_from_index(opcode & 0b111);
    let op16 = operand16_from_index((opcode >> 4) & 0b11);

    return match opcode {
        0x0a => Some(LD(Op8::Reg(A), Op8::AddrBC)),
        0x1a => Some(LD(Op8::Reg(A), Op8::AddrDE)),
        0x02 => Some(LD(Op8::AddrBC, Op8::Reg(A))),
        0x12 => Some(LD(Op8::AddrDE, Op8::Reg(A))),
        0xfa => Some(LD(Op8::Reg(A), Op8::AddrNN)),
        0xea => Some(LD(Op8::AddrNN, Op8::Reg(A))),
        0x36 => Some(LD(Op8::AddrHL, Op8::N)),
        0x22 => Some(LD(Op8::AddrHLInc, Op8::Reg(A))),
        0x2a => Some(LD(Op8::Reg(A), Op8::AddrHLInc)),
        0x32 => Some(LD(Op8::AddrHLDec, Op8::Reg(A))),
        0x3a => Some(LD(Op8::Reg(A), Op8::AddrHLDec)),
        0xe0 => Some(LDH(Op8::AddrN, Op8::Reg(A))),
        0xe2 => Some(LDH(Op8::AddrC, Op8::Reg(A))),
        0xf0 => Some(LDH(Op8::Reg(A), Op8::AddrN)),
        0xf2 => Some(LDH(Op8::Reg(A), Op8::AddrC)),
        0x08 => Some(LD16(Op16::AddrNN, Op16::Reg(SP))),
        0xf9 => Some(LD16(Op16::Reg(SP), Op16::Reg(HL))),
        o if o & 0b11111000 == 0x70 => Some(LD(Op8::AddrHL, op8_from)),
        o if o & 0b11000111 == 0x45 => Some(LD(op8_to, Op8::AddrHL)),
        o if o & 0b11000111 == 0x06 => Some(LD(op8_to, Op8::N)),
        o if o & 0b11000000 == 0x40 => Some(LD(op8_to, op8_from)),
        o if o & 0b11001111 == 0x01 => Some(LD16(op16, Op16::NN)),
        o if o & 0b11001111 == 0xc5 => Some(PUSH16(op16)),
        o if o & 0b11001111 == 0xc1 => Some(POP16(op16)),
        _ => None,
    };
}
