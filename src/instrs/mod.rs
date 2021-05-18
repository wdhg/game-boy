#[allow(dead_code)]
mod alu;
pub mod instr;
mod load;
mod misc;

use instr::Instr;

const PREFIX: u8 = 0xcb;

pub fn decode_unprefixed(opcode: u8) -> Instr {
    let maybe_instr = misc::decode(opcode)
        .or_else(|| load::decode(opcode))
        .or_else(|| alu::decode_unprefixed(opcode));
    match maybe_instr {
        Some(i) => return i,
        None => panic!("Illegal opcode {:#02x}", opcode),
    }
}

pub fn decode_prefixed(opcode: u8) -> Instr {
    let maybe_instr = alu::decode_prefixed(opcode);
    match maybe_instr {
        Some(i) => return i,
        None => panic!("Illegal prefixed opcode {:#02x}", opcode),
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use crate::gameboy::{Reg16, Reg8};
    use instr::Instr::*;
    use instr::Operand::*;
    use Reg16::*;
    use Reg8::*;

    #[test]
    fn decode_nop() {
        assert_eq!(decode_unprefixed(0x00), NOP);
    }

    #[test]
    fn decode_daa() {
        assert_eq!(decode_unprefixed(0x27), DAA);
    }

    #[test]
    fn decode_halt() {
        assert_eq!(decode_unprefixed(0x76), HALT);
    }

    #[test]
    fn decode_stop() {
        assert_eq!(decode_unprefixed(0x10), STOP);
    }

    #[test]
    fn decode_loads_to_registers_from_n() {
        // LD r, n
        assert_eq!(decode_unprefixed(0x06), LD(R8(B), N));
        assert_eq!(decode_unprefixed(0x1e), LD(R8(E), N));
    }

    #[test]
    fn decode_loads_between_registers() {
        // LD r, r'
        assert_eq!(decode_unprefixed(0x41), LD(R8(B), R8(C)));
        assert_eq!(decode_unprefixed(0x6c), LD(R8(L), R8(H)));
    }

    #[test]
    fn decode_loads_between_register_a_and_joined_registers_address() {
        assert_eq!(decode_unprefixed(0x0a), LD(R8(A), AddressBC)); // LD A, (BC)
        assert_eq!(decode_unprefixed(0x1a), LD(R8(A), AddressDE)); // LD A, (DE)
        assert_eq!(decode_unprefixed(0x02), LD(AddressBC, R8(A))); // LD (BC), A
        assert_eq!(decode_unprefixed(0x12), LD(AddressDE, R8(A))); // LD (DE), A
    }

    #[test]
    fn decode_loads_between_register_a_and_address_nn() {
        assert_eq!(decode_unprefixed(0xfa), LD(R8(A), AddressNN)); // LD A, (nn)
        assert_eq!(decode_unprefixed(0xea), LD(AddressNN, R8(A))); // LD (nn), A
    }

    #[test]
    fn decode_loads_to_address_hl_from_n() {
        assert_eq!(decode_unprefixed(0x36), LD(AddressHL, N)); // LD (HL), n
    }

    #[test]
    fn decode_loads_between_r_and_address_hl() {
        assert_eq!(decode_unprefixed(0x46), LD(R8(B), AddressHL)); // LD r, (HL)
        assert_eq!(decode_unprefixed(0x5e), LD(R8(E), AddressHL)); // LD r, (HL)
        assert_eq!(decode_unprefixed(0x70), LD(AddressHL, R8(B))); // LD (HL), r
        assert_eq!(decode_unprefixed(0x77), LD(AddressHL, R8(A))); // LD (HL), r
    }

    #[test]
    fn decode_loads_between_register_a_and_incremented_address_hl() {
        assert_eq!(decode_unprefixed(0x22), LD(AddressHLIncr, R8(A))); // LD (HL+), A
        assert_eq!(decode_unprefixed(0x2a), LD(R8(A), AddressHLIncr)); // LD A, (HL+)
    }

    #[test]
    fn decode_loads_between_register_a_and_decremented_address_hl() {
        assert_eq!(decode_unprefixed(0x32), LD(AddressHLDecr, R8(A))); // LD (HL-), A
        assert_eq!(decode_unprefixed(0x3a), LD(R8(A), AddressHLDecr)); // LD A, (HL-)
    }

    #[test]
    fn decode_ldh() {
        assert_eq!(decode_unprefixed(0xe0), LDH(AddressN, R8(A))); // LDH (n), A
        assert_eq!(decode_unprefixed(0xe2), LDH(AddressC, R8(A))); // LDH (C), A
        assert_eq!(decode_unprefixed(0xf0), LDH(R8(A), AddressN)); // LDH A, (n)
        assert_eq!(decode_unprefixed(0xf2), LDH(R8(A), AddressC)); // LDH A, (C)
    }

    #[test]
    fn decode_loads_to_16_bit_register_from_nn() {
        // LD rr, nn
        assert_eq!(decode_unprefixed(0x01), LD(R16(BC), NN));
        assert_eq!(decode_unprefixed(0x11), LD(R16(DE), NN));
        assert_eq!(decode_unprefixed(0x21), LD(R16(HL), NN));
        assert_eq!(decode_unprefixed(0x31), LD(R16(SP), NN));
    }

    #[test]
    fn decode_loads_to_address_nn_from_sp() {
        assert_eq!(decode_unprefixed(0x08), LD(AddressNN, R16(SP))); // LD (nn), SP
    }

    #[test]
    fn decode_loads_to_sp_from_hl() {
        assert_eq!(decode_unprefixed(0xf9), LD(R16(SP), R16(HL))); // LD SP, HL
    }

    #[test]
    fn decode_pushing_16_bit_register() {
        // PUSH rr
        assert_eq!(decode_unprefixed(0xc5), PUSH(R16(BC)));
        assert_eq!(decode_unprefixed(0xd5), PUSH(R16(DE)));
        assert_eq!(decode_unprefixed(0xe5), PUSH(R16(HL)));
        assert_eq!(decode_unprefixed(0xf5), PUSH(R16(SP)));
    }

    #[test]
    fn decode_popping_16_bit_register() {
        // POP rr
        assert_eq!(decode_unprefixed(0xc1), POP(R16(BC)));
        assert_eq!(decode_unprefixed(0xd1), POP(R16(DE)));
        assert_eq!(decode_unprefixed(0xe1), POP(R16(HL)));
        assert_eq!(decode_unprefixed(0xf1), POP(R16(SP)));
    }

    #[test]
    fn decode_adding_register_to_register_a() {
        // ADD A, r
        assert_eq!(decode_unprefixed(0x80), ADD(R8(A), R8(B)));
        assert_eq!(decode_unprefixed(0x82), ADD(R8(A), R8(D)));
        assert_eq!(decode_unprefixed(0x85), ADD(R8(A), R8(L)));
    }

    #[test]
    fn decode_adding_address_hl_to_register_a() {
        assert_eq!(decode_unprefixed(0x86), ADD(R8(A), AddressHL)); // ADD A, (HL)
    }

    #[test]
    fn decode_adding_n_to_register_a() {
        // NOTE i am guessing at what '#' in 'LD A, #' means
        assert_eq!(decode_unprefixed(0xc6), ADD(R8(A), N)); // ADD A, #
    }

    #[test]
    fn decode_adding_register_to_register_a_with_carry() {
        // ADC A, r
        assert_eq!(decode_unprefixed(0x8f), ADC(R8(A), R8(A)));
        assert_eq!(decode_unprefixed(0x88), ADC(R8(A), R8(B)));
        assert_eq!(decode_unprefixed(0x8d), ADC(R8(A), R8(L)));
    }

    #[test]
    fn decode_adding_address_hl_to_register_a_with_carry() {
        assert_eq!(decode_unprefixed(0x8e), ADC(R8(A), AddressHL)); // ADC A, (HL)
    }

    #[test]
    fn decode_adding_n_to_register_a_with_carry() {
        assert_eq!(decode_unprefixed(0xce), ADC(R8(A), N)); // ADC A, #
    }

    #[test]
    fn decode_subtracting_register() {
        // SUB r
        assert_eq!(decode_unprefixed(0x90), SUB(R8(B)));
        assert_eq!(decode_unprefixed(0x91), SUB(R8(C)));
        assert_eq!(decode_unprefixed(0x95), SUB(R8(L)));
    }

    #[test]
    fn decode_subtracting_address_hl() {
        assert_eq!(decode_unprefixed(0x96), SUB(AddressHL)); // SUB (HL)
    }

    #[test]
    fn decode_subtracting_n() {
        assert_eq!(decode_unprefixed(0xd6), SUB(N)); // SUB #
    }

    #[test]
    fn decode_subtracting_register_with_carry() {
        // SBC r
        assert_eq!(decode_unprefixed(0x98), SBC(R8(B)));
        assert_eq!(decode_unprefixed(0x99), SBC(R8(C)));
        assert_eq!(decode_unprefixed(0x9d), SBC(R8(L)));
    }

    #[test]
    fn decode_subtracting_address_hl_with_carry() {
        assert_eq!(decode_unprefixed(0x9e), SBC(AddressHL)); // SBC (HL)
    }

    #[test]
    fn decode_anding_register() {
        // AND r
        assert_eq!(decode_unprefixed(0xa0), AND(R8(B)));
        assert_eq!(decode_unprefixed(0xa1), AND(R8(C)));
        assert_eq!(decode_unprefixed(0xa5), AND(R8(L)));
    }

    #[test]
    fn decode_anding_address_hl() {
        assert_eq!(decode_unprefixed(0xa6), AND(AddressHL)); // AND (HL)
    }

    #[test]
    fn decode_anding_n() {
        assert_eq!(decode_unprefixed(0xe6), AND(N)); // AND #
    }

    #[test]
    fn decode_oring_register() {
        // OR r
        assert_eq!(decode_unprefixed(0xb0), OR(R8(B)));
        assert_eq!(decode_unprefixed(0xb1), OR(R8(C)));
        assert_eq!(decode_unprefixed(0xb5), OR(R8(L)));
    }

    #[test]
    fn decode_oring_address_hl() {
        assert_eq!(decode_unprefixed(0xb6), OR(AddressHL)); // OR (HL)
    }

    #[test]
    fn decode_oring_n() {
        assert_eq!(decode_unprefixed(0xf6), OR(N)); // OR #
    }

    #[test]
    fn decode_xoring_register() {
        // XOR r
        assert_eq!(decode_unprefixed(0xa8), XOR(R8(B)));
        assert_eq!(decode_unprefixed(0xa9), XOR(R8(C)));
        assert_eq!(decode_unprefixed(0xad), XOR(R8(L)));
    }

    #[test]
    fn decode_xoring_address_hl() {
        assert_eq!(decode_unprefixed(0xae), XOR(AddressHL)); // XOR (HL)
    }

    #[test]
    fn decode_xoring_n() {
        assert_eq!(decode_unprefixed(0xee), XOR(N)); // XOR #
    }

    #[test]
    fn decode_comparing_register() {
        // CP r
        assert_eq!(decode_unprefixed(0xb8), CP(R8(B)));
        assert_eq!(decode_unprefixed(0xb9), CP(R8(C)));
        assert_eq!(decode_unprefixed(0xbd), CP(R8(L)));
    }

    #[test]
    fn decode_comparing_address_hl() {
        assert_eq!(decode_unprefixed(0xbe), CP(AddressHL)); // CP (HL)
    }

    #[test]
    fn decode_comparing_n() {
        assert_eq!(decode_unprefixed(0xfe), CP(N)); // CP #
    }

    #[test]
    fn decode_incrementing_register() {
        // INC r
        assert_eq!(decode_unprefixed(0x3c), INC(R8(A)));
        assert_eq!(decode_unprefixed(0x04), INC(R8(B)));
        assert_eq!(decode_unprefixed(0x0c), INC(R8(C)));
        assert_eq!(decode_unprefixed(0x24), INC(R8(H)));
        assert_eq!(decode_unprefixed(0x2c), INC(R8(L)));
    }

    #[test]
    fn decode_incrementing_address_hl() {
        assert_eq!(decode_unprefixed(0x34), INC(AddressHL)); // INC (HL)
    }

    #[test]
    fn decode_decrementing_register() {
        // DEC r
        assert_eq!(decode_unprefixed(0x3d), DEC(R8(A)));
        assert_eq!(decode_unprefixed(0x05), DEC(R8(B)));
        assert_eq!(decode_unprefixed(0x0d), DEC(R8(C)));
        assert_eq!(decode_unprefixed(0x25), DEC(R8(H)));
        assert_eq!(decode_unprefixed(0x2d), DEC(R8(L)));
    }

    #[test]
    fn decode_decrementing_address_hl() {
        assert_eq!(decode_unprefixed(0x35), DEC(AddressHL)); // DEC (HL)
    }

    #[test]
    fn decode_adding_16_bit_register_to_hl() {
        // ADD HL, rr
        assert_eq!(decode_unprefixed(0x09), ADD(R16(HL), R16(BC)));
        assert_eq!(decode_unprefixed(0x19), ADD(R16(HL), R16(DE)));
        assert_eq!(decode_unprefixed(0x29), ADD(R16(HL), R16(HL)));
        assert_eq!(decode_unprefixed(0x39), ADD(R16(HL), R16(SP)));
    }

    #[test]
    fn decode_adding_n_to_sp() {
        // ADD SP, #
        assert_eq!(decode_unprefixed(0xe8), ADD(R16(SP), N));
    }

    #[test]
    fn decode_incrementing_16_bit_register() {
        // INC rr
        assert_eq!(decode_unprefixed(0x03), INC(R16(BC)));
        assert_eq!(decode_unprefixed(0x13), INC(R16(DE)));
        assert_eq!(decode_unprefixed(0x23), INC(R16(HL)));
        assert_eq!(decode_unprefixed(0x33), INC(R16(SP)));
    }

    #[test]
    fn decode_decrementing_16_bit_register() {
        // DEC rr
        assert_eq!(decode_unprefixed(0x0b), DEC(R16(BC)));
        assert_eq!(decode_unprefixed(0x1b), DEC(R16(DE)));
        assert_eq!(decode_unprefixed(0x2b), DEC(R16(HL)));
        assert_eq!(decode_unprefixed(0x3b), DEC(R16(SP)));
    }

    #[test]
    fn decode_rotating_register_a() {
        assert_eq!(decode_prefixed(0x07), RLCA);
        assert_eq!(decode_prefixed(0x17), RLA);
        assert_eq!(decode_prefixed(0x0f), RRCA);
        assert_eq!(decode_prefixed(0x1f), RRA);
    }
}
