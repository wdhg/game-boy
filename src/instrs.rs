use crate::gameboy::{Reg16, Reg8};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    R(Reg8),       // 8 bit register
    N,             // 8 bit number following opcode
    AddressN,      // 8 bit address following opcode
    AddressC,      // 8 bit address stored in C
    RR(Reg16),     // 16 bit register
    NN,            // 16 bit number following opcode
    AddressNN,     // 16 bit address following opcode
    AddressBC,     // 16 bit address stored in B and C
    AddressDE,     // 16 bit address stored in D and E
    AddressHL,     // 16 bit address stored in H and L
    AddressHLIncr, // 16 bit address stored in H and L, incremented after use
    AddressHLDecr, // 16 bit address stored in H and L, decremented after use
}

fn r_from_index(i: u8) -> Operand {
    return Operand::R(Reg8::from_index(i));
}

fn rr_from_index(i: u8) -> Operand {
    return Operand::RR(Reg16::from_index(i));
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
    ADC(Operand, Operand), // add + carry instruction
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
    use Reg16::*;
    use Reg8::*;

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

fn decode_arithmetic(opcode: u8) -> Option<Instr> {
    use Operand::*;
    use Reg8::*;

    let reg8_from = opcode & 0b111;

    return match opcode {
        0xc6 => Some(ADD(R(A), N)),
        0x86 => Some(ADD(R(A), AddressHL)),
        0xce => Some(ADC(R(A), N)),
        0x8e => Some(ADC(R(A), AddressHL)),
        o if o & 0b11111000 == 0b10000000 => Some(ADD(R(A), r_from_index(reg8_from))),
        o if o & 0b11111000 == 0b10001000 => Some(ADC(R(A), r_from_index(reg8_from))),
        _ => None,
    };
}

#[allow(dead_code)]
pub fn decode(opcode: u8) -> Instr {
    let maybe_instr = decode_misc(opcode)
        .or_else(|| decode_load(opcode))
        .or_else(|| decode_arithmetic(opcode));
    match maybe_instr {
        Some(i) => return i,
        None => panic!("Illegal opcode {:#02x}", opcode),
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use Operand::*;
    use Reg16::*;
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
    fn decode_loads_to_registers_from_n() {
        // LD r, n
        assert_eq!(decode(0x06), LD(R(B), N));
        assert_eq!(decode(0x1e), LD(R(E), N));
    }

    #[test]
    fn decode_loads_between_registers() {
        // LD r, r'
        assert_eq!(decode(0x41), LD(R(B), R(C)));
        assert_eq!(decode(0x6c), LD(R(L), R(H)));
    }

    #[test]
    fn decode_loads_between_register_a_and_joined_registers_address() {
        assert_eq!(decode(0x0a), LD(R(A), AddressBC)); // LD A, (BC)
        assert_eq!(decode(0x1a), LD(R(A), AddressDE)); // LD A, (DE)
        assert_eq!(decode(0x02), LD(AddressBC, R(A))); // LD (BC), A
        assert_eq!(decode(0x12), LD(AddressDE, R(A))); // LD (DE), A
    }

    #[test]
    fn decode_loads_between_register_a_and_address_nn() {
        assert_eq!(decode(0xfa), LD(R(A), AddressNN)); // LD A, (nn)
        assert_eq!(decode(0xea), LD(AddressNN, R(A))); // LD (nn), A
    }

    #[test]
    fn decode_loads_to_address_hl_from_n() {
        assert_eq!(decode(0x36), LD(AddressHL, N)); // LD (HL), n
    }

    #[test]
    fn decode_loads_between_r_and_address_hl() {
        assert_eq!(decode(0x46), LD(R(B), AddressHL)); // LD r, (HL)
        assert_eq!(decode(0x5e), LD(R(E), AddressHL)); // LD r, (HL)
        assert_eq!(decode(0x70), LD(AddressHL, R(B))); // LD (HL), r
        assert_eq!(decode(0x77), LD(AddressHL, R(A))); // LD (HL), r
    }

    #[test]
    fn decode_loads_between_register_a_and_incremented_address_hl() {
        assert_eq!(decode(0x22), LD(AddressHLIncr, R(A))); // LD (HL+), A
        assert_eq!(decode(0x2a), LD(R(A), AddressHLIncr)); // LD A, (HL+)
    }

    #[test]
    fn decode_loads_between_register_a_and_decremented_address_hl() {
        assert_eq!(decode(0x32), LD(AddressHLDecr, R(A))); // LD (HL-), A
        assert_eq!(decode(0x3a), LD(R(A), AddressHLDecr)); // LD A, (HL-)
    }

    #[test]
    fn decode_ldh() {
        assert_eq!(decode(0xe0), LDH(AddressN, R(A))); // LDH (n), A
        assert_eq!(decode(0xe2), LDH(AddressC, R(A))); // LDH (C), A
        assert_eq!(decode(0xf0), LDH(R(A), AddressN)); // LDH A, (n)
        assert_eq!(decode(0xf2), LDH(R(A), AddressC)); // LDH A, (C)
    }

    #[test]
    fn decode_loads_to_16_bit_register_from_nn() {
        // LD rr, nn
        assert_eq!(decode(0x01), LD(RR(BC), NN));
        assert_eq!(decode(0x11), LD(RR(DE), NN));
        assert_eq!(decode(0x21), LD(RR(HL), NN));
        assert_eq!(decode(0x31), LD(RR(SP), NN));
    }

    #[test]
    fn decode_loads_to_address_nn_from_sp() {
        assert_eq!(decode(0x08), LD(AddressNN, RR(SP))); // LD (nn), SP
    }

    #[test]
    fn decode_loads_to_sp_from_hl() {
        assert_eq!(decode(0xf9), LD(RR(SP), RR(HL))); // LD SP, HL
    }

    #[test]
    fn decode_pushing_16_bit_register() {
        // PUSH rr
        assert_eq!(decode(0xc5), PUSH(RR(BC)));
        assert_eq!(decode(0xd5), PUSH(RR(DE)));
        assert_eq!(decode(0xe5), PUSH(RR(HL)));
        assert_eq!(decode(0xf5), PUSH(RR(SP)));
    }

    #[test]
    fn decode_popping_16_bit_register() {
        // POP rr
        assert_eq!(decode(0xc1), POP(RR(BC)));
        assert_eq!(decode(0xd1), POP(RR(DE)));
        assert_eq!(decode(0xe1), POP(RR(HL)));
        assert_eq!(decode(0xf1), POP(RR(SP)));
    }

    #[test]
    fn decode_adding_register_to_register_a() {
        // ADD A, r
        assert_eq!(decode(0x80), ADD(R(A), R(B)));
        assert_eq!(decode(0x82), ADD(R(A), R(D)));
        assert_eq!(decode(0x85), ADD(R(A), R(L)));
    }

    #[test]
    fn decode_adding_address_hl_to_register_a() {
        assert_eq!(decode(0x86), ADD(R(A), AddressHL)); // ADD A, (HL)
    }

    #[test]
    fn decode_adding_n_to_register_a() {
        // NOTE i am guessing at what '#' in 'LD A, #' means
        assert_eq!(decode(0xc6), ADD(R(A), N)); // ADD A, #
    }

    #[test]
    fn decode_adcing_register_to_register_a() {
        // ADC A, r
        assert_eq!(decode(0x8f), ADC(R(A), R(A)));
        assert_eq!(decode(0x88), ADC(R(A), R(B)));
        assert_eq!(decode(0x8d), ADC(R(A), R(L)));
    }

    #[test]
    fn decode_adcing_address_hl_to_register_a() {
        assert_eq!(decode(0x8e), ADC(R(A), AddressHL)); // ADC A, (HL)
    }

    #[test]
    fn decode_adcing_n_to_register_a() {
        assert_eq!(decode(0xce), ADC(R(A), N)); // ADC A, #
    }
}
