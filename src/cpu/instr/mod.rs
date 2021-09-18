mod alu;
pub mod instr;
mod load;
mod misc;
pub mod operand;

use instr::Instr;

#[allow(dead_code)]
pub const PREFIX: u8 = 0xcb;

#[allow(dead_code)]
pub fn decode_unprefixed(opcode: u8) -> Instr {
    let maybe_instr = misc::decode_unprefixed(opcode)
        .or_else(|| load::decode_unprefixed(opcode))
        .or_else(|| alu::decode_unprefixed(opcode));
    match maybe_instr {
        Some(i) => return i,
        None => panic!("Illegal opcode {:#02x}", opcode),
    }
}

#[allow(dead_code)]
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
    use crate::cpu::instr::operand::{Op16, Op8};
    use crate::cpu::instr::Instr::*;
    use crate::cpu::{Reg16::*, Reg8::*};

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
        assert_eq!(decode_unprefixed(0x06), LD(Op8::Reg(B), Op8::N));
        assert_eq!(decode_unprefixed(0x1e), LD(Op8::Reg(E), Op8::N));
    }

    #[test]
    fn decode_loads_between_registers() {
        // LD r, r'
        assert_eq!(decode_unprefixed(0x41), LD(Op8::Reg(B), Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0x6c), LD(Op8::Reg(L), Op8::Reg(H)));
    }

    #[test]
    fn decode_loads_between_register_a_and_joined_registers_address() {
        assert_eq!(decode_unprefixed(0x0a), LD(Op8::Reg(A), Op8::AddrBC)); // LD A, (BC)
        assert_eq!(decode_unprefixed(0x1a), LD(Op8::Reg(A), Op8::AddrDE)); // LD A, (DE)
        assert_eq!(decode_unprefixed(0x02), LD(Op8::AddrBC, Op8::Reg(A))); // LD (BC), A
        assert_eq!(decode_unprefixed(0x12), LD(Op8::AddrDE, Op8::Reg(A))); // LD (DE), A
    }

    #[test]
    fn decode_loads_between_register_a_and_address_nn() {
        assert_eq!(decode_unprefixed(0xfa), LD(Op8::Reg(A), Op8::AddrNN)); // LD A, (nn)
        assert_eq!(decode_unprefixed(0xea), LD(Op8::AddrNN, Op8::Reg(A))); // LD (nn), A
    }

    #[test]
    fn decode_loads_to_address_hl_from_n() {
        assert_eq!(decode_unprefixed(0x36), LD(Op8::AddrHL, Op8::N)); // LD (HL), n
    }

    #[test]
    fn decode_loads_between_r_and_address_hl() {
        assert_eq!(decode_unprefixed(0x46), LD(Op8::Reg(B), Op8::AddrHL)); // LD r, (HL)
        assert_eq!(decode_unprefixed(0x5e), LD(Op8::Reg(E), Op8::AddrHL)); // LD r, (HL)
        assert_eq!(decode_unprefixed(0x70), LD(Op8::AddrHL, Op8::Reg(B))); // LD (HL), r
        assert_eq!(decode_unprefixed(0x77), LD(Op8::AddrHL, Op8::Reg(A))); // LD (HL), r
    }

    #[test]
    fn decode_loads_between_register_a_and_incremented_address_hl() {
        assert_eq!(decode_unprefixed(0x22), LD(Op8::AddrHLInc, Op8::Reg(A))); // LD (HL+), A
        assert_eq!(decode_unprefixed(0x2a), LD(Op8::Reg(A), Op8::AddrHLInc)); // LD A, (HL+)
    }

    #[test]
    fn decode_loads_between_register_a_and_decremented_address_hl() {
        assert_eq!(decode_unprefixed(0x32), LD(Op8::AddrHLDec, Op8::Reg(A))); // LD (HL-), A
        assert_eq!(decode_unprefixed(0x3a), LD(Op8::Reg(A), Op8::AddrHLDec)); // LD A, (HL-)
    }

    #[test]
    fn decode_ldh() {
        assert_eq!(decode_unprefixed(0xe0), LDH(Op8::AddrN, Op8::Reg(A))); // LDH (n), A
        assert_eq!(decode_unprefixed(0xe2), LDH(Op8::AddrC, Op8::Reg(A))); // LDH (C), A
        assert_eq!(decode_unprefixed(0xf0), LDH(Op8::Reg(A), Op8::AddrN)); // LDH A, (n)
        assert_eq!(decode_unprefixed(0xf2), LDH(Op8::Reg(A), Op8::AddrC)); // LDH A, (C)
    }

    #[test]
    fn decode_loads_to_16_bit_register_from_nn() {
        // LD rr, nn
        assert_eq!(decode_unprefixed(0x01), LD16(Op16::Reg(BC), Op16::NN));
        assert_eq!(decode_unprefixed(0x11), LD16(Op16::Reg(DE), Op16::NN));
        assert_eq!(decode_unprefixed(0x21), LD16(Op16::Reg(HL), Op16::NN));
        assert_eq!(decode_unprefixed(0x31), LD16(Op16::Reg(SP), Op16::NN));
    }

    #[test]
    fn decode_loads_to_address_nn_from_sp() {
        assert_eq!(decode_unprefixed(0x08), LD16(Op16::AddrNN, Op16::Reg(SP))); // LD (nn), SP
    }

    #[test]
    fn decode_loads_to_sp_from_hl() {
        assert_eq!(decode_unprefixed(0xf9), LD16(Op16::Reg(SP), Op16::Reg(HL)));
        // LD SP, HL
    }

    #[test]
    fn decode_pushing_16_bit_register() {
        // PUSH rr
        assert_eq!(decode_unprefixed(0xc5), PUSH(Op16::Reg(BC)));
        assert_eq!(decode_unprefixed(0xd5), PUSH(Op16::Reg(DE)));
        assert_eq!(decode_unprefixed(0xe5), PUSH(Op16::Reg(HL)));
        assert_eq!(decode_unprefixed(0xf5), PUSH(Op16::Reg(SP)));
    }

    #[test]
    fn decode_popping_16_bit_register() {
        // POP rr
        assert_eq!(decode_unprefixed(0xc1), POP(Op16::Reg(BC)));
        assert_eq!(decode_unprefixed(0xd1), POP(Op16::Reg(DE)));
        assert_eq!(decode_unprefixed(0xe1), POP(Op16::Reg(HL)));
        assert_eq!(decode_unprefixed(0xf1), POP(Op16::Reg(SP)));
    }

    #[test]
    fn decode_adding_register_to_register_a() {
        // ADD A, r
        assert_eq!(decode_unprefixed(0x80), ADD(Op8::Reg(A), Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0x82), ADD(Op8::Reg(A), Op8::Reg(D)));
        assert_eq!(decode_unprefixed(0x85), ADD(Op8::Reg(A), Op8::Reg(L)));
    }

    #[test]
    fn decode_adding_address_hl_to_register_a() {
        assert_eq!(decode_unprefixed(0x86), ADD(Op8::Reg(A), Op8::AddrHL)); // ADD A, (HL)
    }

    #[test]
    fn decode_adding_n_to_register_a() {
        // NOTE i am guessing at what '#' in 'LD A, #' means
        assert_eq!(decode_unprefixed(0xc6), ADD(Op8::Reg(A), Op8::N)); // ADD A, #
    }

    #[test]
    fn decode_adding_register_to_register_a_with_carry() {
        // ADC A, r
        assert_eq!(decode_unprefixed(0x8f), ADC(Op8::Reg(A), Op8::Reg(A)));
        assert_eq!(decode_unprefixed(0x88), ADC(Op8::Reg(A), Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0x8d), ADC(Op8::Reg(A), Op8::Reg(L)));
    }

    #[test]
    fn decode_adding_address_hl_to_register_a_with_carry() {
        assert_eq!(decode_unprefixed(0x8e), ADC(Op8::Reg(A), Op8::AddrHL)); // ADC A, (HL)
    }

    #[test]
    fn decode_adding_n_to_register_a_with_carry() {
        assert_eq!(decode_unprefixed(0xce), ADC(Op8::Reg(A), Op8::N)); // ADC A, #
    }

    #[test]
    fn decode_subtracting_register() {
        // SUB r
        assert_eq!(decode_unprefixed(0x90), SUB(Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0x91), SUB(Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0x95), SUB(Op8::Reg(L)));
    }

    #[test]
    fn decode_subtracting_address_hl() {
        assert_eq!(decode_unprefixed(0x96), SUB(Op8::AddrHL)); // SUB (HL)
    }

    #[test]
    fn decode_subtracting_n() {
        assert_eq!(decode_unprefixed(0xd6), SUB(Op8::N)); // SUB #
    }

    #[test]
    fn decode_subtracting_register_with_carry() {
        // SBC r
        assert_eq!(decode_unprefixed(0x98), SBC(Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0x99), SBC(Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0x9d), SBC(Op8::Reg(L)));
    }

    #[test]
    fn decode_subtracting_address_hl_with_carry() {
        assert_eq!(decode_unprefixed(0x9e), SBC(Op8::AddrHL)); // SBC (HL)
    }

    #[test]
    fn decode_anding_register() {
        // AND r
        assert_eq!(decode_unprefixed(0xa0), AND(Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0xa1), AND(Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0xa5), AND(Op8::Reg(L)));
    }

    #[test]
    fn decode_anding_address_hl() {
        assert_eq!(decode_unprefixed(0xa6), AND(Op8::AddrHL)); // AND (HL)
    }

    #[test]
    fn decode_anding_n() {
        assert_eq!(decode_unprefixed(0xe6), AND(Op8::N)); // AND #
    }

    #[test]
    fn decode_oring_register() {
        // OR r
        assert_eq!(decode_unprefixed(0xb0), OR(Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0xb1), OR(Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0xb5), OR(Op8::Reg(L)));
    }

    #[test]
    fn decode_oring_address_hl() {
        assert_eq!(decode_unprefixed(0xb6), OR(Op8::AddrHL)); // OR (HL)
    }

    #[test]
    fn decode_oring_n() {
        assert_eq!(decode_unprefixed(0xf6), OR(Op8::N)); // OR #
    }

    #[test]
    fn decode_xoring_register() {
        // XOR r
        assert_eq!(decode_unprefixed(0xa8), XOR(Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0xa9), XOR(Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0xad), XOR(Op8::Reg(L)));
    }

    #[test]
    fn decode_xoring_address_hl() {
        assert_eq!(decode_unprefixed(0xae), XOR(Op8::AddrHL)); // XOR (HL)
    }

    #[test]
    fn decode_xoring_n() {
        assert_eq!(decode_unprefixed(0xee), XOR(Op8::N)); // XOR #
    }

    #[test]
    fn decode_comparing_register() {
        // CP r
        assert_eq!(decode_unprefixed(0xb8), CP(Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0xb9), CP(Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0xbd), CP(Op8::Reg(L)));
    }

    #[test]
    fn decode_comparing_address_hl() {
        assert_eq!(decode_unprefixed(0xbe), CP(Op8::AddrHL)); // CP (HL)
    }

    #[test]
    fn decode_comparing_n() {
        assert_eq!(decode_unprefixed(0xfe), CP(Op8::N)); // CP #
    }

    #[test]
    fn decode_incrementing_register() {
        // INC r
        assert_eq!(decode_unprefixed(0x3c), INC(Op8::Reg(A)));
        assert_eq!(decode_unprefixed(0x04), INC(Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0x0c), INC(Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0x24), INC(Op8::Reg(H)));
        assert_eq!(decode_unprefixed(0x2c), INC(Op8::Reg(L)));
    }

    #[test]
    fn decode_incrementing_address_hl() {
        assert_eq!(decode_unprefixed(0x34), INC(Op8::AddrHL)); // INC (HL)
    }

    #[test]
    fn decode_decrementing_register() {
        // DEC r
        assert_eq!(decode_unprefixed(0x3d), DEC(Op8::Reg(A)));
        assert_eq!(decode_unprefixed(0x05), DEC(Op8::Reg(B)));
        assert_eq!(decode_unprefixed(0x0d), DEC(Op8::Reg(C)));
        assert_eq!(decode_unprefixed(0x25), DEC(Op8::Reg(H)));
        assert_eq!(decode_unprefixed(0x2d), DEC(Op8::Reg(L)));
    }

    #[test]
    fn decode_decrementing_address_hl() {
        assert_eq!(decode_unprefixed(0x35), DEC(Op8::AddrHL)); // DEC (HL)
    }

    #[test]
    fn decode_adding_16_bit_register_to_hl() {
        // ADD HL, rr
        assert_eq!(decode_unprefixed(0x09), ADD16(Op16::Reg(HL), Op16::Reg(BC)));
        assert_eq!(decode_unprefixed(0x19), ADD16(Op16::Reg(HL), Op16::Reg(DE)));
        assert_eq!(decode_unprefixed(0x29), ADD16(Op16::Reg(HL), Op16::Reg(HL)));
        assert_eq!(decode_unprefixed(0x39), ADD16(Op16::Reg(HL), Op16::Reg(SP)));
    }

    #[test]
    fn decode_adding_n_to_sp() {
        // ADD SP, #
        assert_eq!(decode_unprefixed(0xe8), ADD16(Op16::Reg(SP), Op16::N));
    }

    #[test]
    fn decode_incrementing_16_bit_register() {
        // INC rr
        assert_eq!(decode_unprefixed(0x03), INC16(Op16::Reg(BC)));
        assert_eq!(decode_unprefixed(0x13), INC16(Op16::Reg(DE)));
        assert_eq!(decode_unprefixed(0x23), INC16(Op16::Reg(HL)));
        assert_eq!(decode_unprefixed(0x33), INC16(Op16::Reg(SP)));
    }

    #[test]
    fn decode_decrementing_16_bit_register() {
        // DEC rr
        assert_eq!(decode_unprefixed(0x0b), DEC16(Op16::Reg(BC)));
        assert_eq!(decode_unprefixed(0x1b), DEC16(Op16::Reg(DE)));
        assert_eq!(decode_unprefixed(0x2b), DEC16(Op16::Reg(HL)));
        assert_eq!(decode_unprefixed(0x3b), DEC16(Op16::Reg(SP)));
    }

    #[test]
    fn decode_rotating_left_carry() {
        assert_eq!(decode_unprefixed(0x07), RLC(Op8::Reg(A)));

        assert_eq!(decode_prefixed(0x05), RLC(Op8::Reg(L)));
        assert_eq!(decode_prefixed(0x06), RLC(Op8::AddrHL));
    }

    #[test]
    fn decode_rotating_left() {
        assert_eq!(decode_unprefixed(0x17), RL(Op8::Reg(A)));

        assert_eq!(decode_prefixed(0x12), RL(Op8::Reg(D)));
        assert_eq!(decode_prefixed(0x16), RL(Op8::AddrHL));
    }

    #[test]
    fn decode_rotating_right_carry() {
        assert_eq!(decode_unprefixed(0x0f), RRC(Op8::Reg(A)));

        assert_eq!(decode_prefixed(0x09), RRC(Op8::Reg(C)));
        assert_eq!(decode_prefixed(0x0e), RRC(Op8::AddrHL));
    }

    #[test]
    fn decode_rotating_right() {
        assert_eq!(decode_unprefixed(0x1f), RR(Op8::Reg(A)));

        assert_eq!(decode_prefixed(0x1a), RR(Op8::Reg(D)));
        assert_eq!(decode_prefixed(0x1e), RR(Op8::AddrHL));
    }

    #[test]
    fn decode_shift_left() {
        assert_eq!(decode_prefixed(0x27), SLA(Op8::Reg(A)));
        assert_eq!(decode_prefixed(0x25), SLA(Op8::Reg(L)));
        assert_eq!(decode_prefixed(0x26), SLA(Op8::AddrHL));
    }

    #[test]
    fn decode_shift_right_msb_unchanging() {
        assert_eq!(decode_prefixed(0x2f), SRA(Op8::Reg(A)));
        assert_eq!(decode_prefixed(0x29), SRA(Op8::Reg(C)));
        assert_eq!(decode_prefixed(0x2e), SRA(Op8::AddrHL));
    }

    #[test]
    fn decode_shift_right_msb_unset() {
        assert_eq!(decode_prefixed(0x3f), SRL(Op8::Reg(A)));
        assert_eq!(decode_prefixed(0x3c), SRL(Op8::Reg(H)));
        assert_eq!(decode_prefixed(0x3e), SRL(Op8::AddrHL));
    }

    #[test]
    fn decode_bit_testing() {
        assert_eq!(decode_prefixed(0x72), BIT(6, Op8::Reg(D)));
        assert_eq!(decode_prefixed(0x6f), BIT(5, Op8::Reg(A)));
        assert_eq!(decode_prefixed(0x5e), BIT(3, Op8::AddrHL));
    }

    #[test]
    fn decode_bit_setting() {
        assert_eq!(decode_prefixed(0xc3), SET(0, Op8::Reg(E)));
        assert_eq!(decode_prefixed(0xe5), SET(4, Op8::Reg(L)));
        assert_eq!(decode_prefixed(0xee), SET(5, Op8::AddrHL));
    }

    #[test]
    fn decode_bit_unsetting() {
        assert_eq!(decode_prefixed(0x81), RES(0, Op8::Reg(C)));
        assert_eq!(decode_prefixed(0xa2), RES(4, Op8::Reg(D)));
        assert_eq!(decode_prefixed(0xbe), RES(7, Op8::AddrHL));
    }
}
