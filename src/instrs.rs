use crate::gameboy::Reg8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    R(Reg8),       // 8 bit register
    N,             // 8 bit number following opcode
    AddressN,      // 8 bit address following opcode
    AddressC,      // 8 bit address stored in C
    NN,            // 16 bit number following opcode
    AddressNN,     // 16 bit address following opcode
    AddressBC,     // 16 bit address stored in B and C
    AddressDE,     // 16 bit address stored in D and E
    AddressHL,     // 16 bit address stored in H and L
    AddressHLIncr, // 16 bit address stored in H and L, incremented after use
    AddressHLDecr, // 16 bit address stored in H and L, decremented after use
    AddressSP,     // 16 bit address stored in SP
}

fn r_from_index(i: u8) -> Operand {
    return Operand::R(Reg8::from_index(i));
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
}

use Instr::*;

fn decode_misc(opcode: u8) -> Option<Instr> {
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

fn decode_load(opcode: u8) -> Option<Instr> {
    use Operand::*;

    let reg_to = (opcode >> 3) & 0b111;
    let reg_from = opcode & 0b111;

    return match opcode {
        0x36 => Some(LD(AddressHL, N)),
        o if o & 0b11111000 == 0b01110000 => Some(LD(AddressHL, r_from_index(reg_from))),
        o if o & 0b11000111 == 0b01000110 => Some(LD(r_from_index(reg_to), AddressHL)),
        o if o & 0b11000111 == 0b00000110 => Some(LD(r_from_index(reg_to), N)),
        o if o & 0b11000000 == 0b01000000 => Some(LD(r_from_index(reg_to), r_from_index(reg_from))),
        _ => None,
    };
}

#[allow(dead_code)]
pub fn decode(opcode: u8) -> Instr {
    let maybe_instr = decode_misc(opcode).or_else(|| decode_load(opcode));
    match maybe_instr {
        Some(i) => return i,
        None => panic!("Illegal opcode {:#02x}", opcode),
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use Operand::*;
    use Reg8::*;

    #[test]
    fn decode_nop() {
        assert_eq!(decode(0x00), NOP);
    }

    #[test]
    fn decode_daa() {
        assert_eq!(decode(0x27), DAA);
    }

    #[test]
    fn decode_halt() {
        assert_eq!(decode(0x76), HALT);
    }

    #[test]
    fn decode_stop() {
        assert_eq!(decode(0x10), STOP);
    }

    #[test]
    fn decode_ld() {
        assert_eq!(decode(0x36), LD(AddressHL, N));
        assert_eq!(decode(0x41), LD(R(B), R(C)));
        assert_eq!(decode(0x6c), LD(R(L), R(H)));
        assert_eq!(decode(0x06), LD(R(B), N));
        assert_eq!(decode(0x1e), LD(R(E), N));
        assert_eq!(decode(0x46), LD(R(B), AddressHL));
        assert_eq!(decode(0x5e), LD(R(E), AddressHL));
        assert_eq!(decode(0x70), LD(AddressHL, R(B)));
        assert_eq!(decode(0x77), LD(AddressHL, R(A)));
    }
}
