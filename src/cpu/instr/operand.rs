use crate::cpu::{Reg16, Reg8};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op8 {
    Reg(Reg8),
    AddrN,
    AddrC,
    AddrNN,
    AddrBC,
    AddrDE,
    AddrHL,
    AddrHLInc, // increment HL after memory access
    AddrHLDec, // decrement HL after memory access
    N,         // following byte
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op16 {
    Reg(Reg16),
    AddrN,
    AddrC,
    AddrNN,
    AddrBC,
    AddrDE,
    AddrHL,
    AddrHLInc, // increment HL after memory access
    AddrHLDec, // decrement HL after memory access
    N,         // following byte
    NN,        // following 2 bytes
}

pub(crate) fn operand8_from_index(i: u8) -> Op8 {
    if i == 6 {
        return Op8::AddrHL;
    }
    return Op8::Reg(Reg8::from_index(i));
}

pub(crate) fn operand16_from_index(i: u8) -> Op16 {
    return Op16::Reg(Reg16::from_index(i));
}
