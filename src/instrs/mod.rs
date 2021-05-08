mod alu;
pub mod instr;
mod load;
mod misc;

use instr::Instr;

#[allow(dead_code)]
pub fn decode(opcode: u8) -> Instr {
    let maybe_instr = misc::decode(opcode)
        .or_else(|| load::decode(opcode))
        .or_else(|| alu::decode(opcode));
    match maybe_instr {
        Some(i) => return i,
        None => panic!("Illegal opcode {:#02x}", opcode),
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
    fn decode_adding_register_to_register_a_with_carry() {
        // ADC A, r
        assert_eq!(decode(0x8f), ADC(R(A), R(A)));
        assert_eq!(decode(0x88), ADC(R(A), R(B)));
        assert_eq!(decode(0x8d), ADC(R(A), R(L)));
    }

    #[test]
    fn decode_adding_address_hl_to_register_a_with_carry() {
        assert_eq!(decode(0x8e), ADC(R(A), AddressHL)); // ADC A, (HL)
    }

    #[test]
    fn decode_adding_n_to_register_a_with_carry() {
        assert_eq!(decode(0xce), ADC(R(A), N)); // ADC A, #
    }

    #[test]
    fn decode_subtracting_register() {
        // SUB r
        assert_eq!(decode(0x90), SUB(R(B)));
        assert_eq!(decode(0x91), SUB(R(C)));
        assert_eq!(decode(0x95), SUB(R(L)));
    }

    #[test]
    fn decode_subtracting_address_hl() {
        assert_eq!(decode(0x96), SUB(AddressHL)); // SUB (HL)
    }

    #[test]
    fn decode_subtracting_n() {
        assert_eq!(decode(0xd6), SUB(N)); // SUB #
    }

    #[test]
    fn decode_subtracting_register_with_carry() {
        // SBC r
        assert_eq!(decode(0x98), SBC(R(B)));
        assert_eq!(decode(0x99), SBC(R(C)));
        assert_eq!(decode(0x9d), SBC(R(L)));
    }

    #[test]
    fn decode_subtracting_address_hl_with_carry() {
        assert_eq!(decode(0x9e), SBC(AddressHL)); // SBC (HL)
    }

    #[test]
    fn decode_anding_register() {
        // AND r
        assert_eq!(decode(0xa0), AND(R(B)));
        assert_eq!(decode(0xa1), AND(R(C)));
        assert_eq!(decode(0xa5), AND(R(L)));
    }

    #[test]
    fn decode_anding_address_hl() {
        assert_eq!(decode(0xa6), AND(AddressHL)); // AND (HL)
    }

    #[test]
    fn decode_anding_n() {
        assert_eq!(decode(0xe6), AND(N)); // AND #
    }

    #[test]
    fn decode_oring_register() {
        // OR r
        assert_eq!(decode(0xb0), OR(R(B)));
        assert_eq!(decode(0xb1), OR(R(C)));
        assert_eq!(decode(0xb5), OR(R(L)));
    }

    #[test]
    fn decode_oring_address_hl() {
        assert_eq!(decode(0xb6), OR(AddressHL)); // OR (HL)
    }

    #[test]
    fn decode_oring_n() {
        assert_eq!(decode(0xf6), OR(N)); // OR #
    }

    #[test]
    fn decode_xoring_register() {
        // XOR r
        assert_eq!(decode(0xa8), XOR(R(B)));
        assert_eq!(decode(0xa9), XOR(R(C)));
        assert_eq!(decode(0xad), XOR(R(L)));
    }

    #[test]
    fn decode_xoring_address_hl() {
        assert_eq!(decode(0xae), XOR(AddressHL)); // XOR (HL)
    }

    #[test]
    fn decode_xoring_n() {
        assert_eq!(decode(0xee), XOR(N)); // XOR #
    }

    #[test]
    fn decode_comparing_register() {
        // CP r
        assert_eq!(decode(0xb8), CP(R(B)));
        assert_eq!(decode(0xb9), CP(R(C)));
        assert_eq!(decode(0xbd), CP(R(L)));
    }

    #[test]
    fn decode_comparing_address_hl() {
        assert_eq!(decode(0xbe), CP(AddressHL)); // CP (HL)
    }

    #[test]
    fn decode_comparing_n() {
        assert_eq!(decode(0xfe), CP(N)); // CP #
    }

    #[test]
    fn decode_incrementing_register() {
        // INC r
        assert_eq!(decode(0x3c), INC(R(A)));
        assert_eq!(decode(0x04), INC(R(B)));
        assert_eq!(decode(0x0c), INC(R(C)));
        assert_eq!(decode(0x24), INC(R(H)));
        assert_eq!(decode(0x2c), INC(R(L)));
    }

    #[test]
    fn decode_incrementing_address_hl() {
        assert_eq!(decode(0x34), INC(AddressHL)); // INC (HL)
    }

    #[test]
    fn decode_decrementing_register() {
        // DEC r
        assert_eq!(decode(0x3d), DEC(R(A)));
        assert_eq!(decode(0x05), DEC(R(B)));
        assert_eq!(decode(0x0d), DEC(R(C)));
        assert_eq!(decode(0x25), DEC(R(H)));
        assert_eq!(decode(0x2d), DEC(R(L)));
    }

    #[test]
    fn decode_decrementing_address_hl() {
        assert_eq!(decode(0x35), DEC(AddressHL)); // DEC (HL)
    }

    #[test]
    fn decode_adding_16_bit_register_to_hl() {
        // ADD HL, rr
        assert_eq!(decode(0x09), ADD(RR(HL), RR(BC)));
        assert_eq!(decode(0x19), ADD(RR(HL), RR(DE)));
        assert_eq!(decode(0x29), ADD(RR(HL), RR(HL)));
        assert_eq!(decode(0x39), ADD(RR(HL), RR(SP)));
    }

    #[test]
    fn decode_adding_n_to_sp() {
        // ADD SP, #
        assert_eq!(decode(0xe8), ADD(RR(SP), N));
    }

    #[test]
    fn decode_incrementing_16_bit_register() {
        // INC rr
        assert_eq!(decode(0x03), INC(RR(BC)));
        assert_eq!(decode(0x13), INC(RR(DE)));
        assert_eq!(decode(0x23), INC(RR(HL)));
        assert_eq!(decode(0x33), INC(RR(SP)));
    }
}
