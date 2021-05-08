use super::instr::{Instr, Instr::*};

pub(crate) fn decode(opcode: u8) -> Option<Instr> {
    return match opcode {
        0x00 => Some(NOP),
        0x10 => Some(STOP),
        0x27 => Some(DAA),
        0x2f => Some(CPL),
        0x3f => Some(CCF),
        0x37 => Some(SCF),
        0x76 => Some(HALT),
        0xf3 => Some(DI),
        0xfb => Some(EI),
        _ => None,
    };
}
