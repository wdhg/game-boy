use crate::gameboy::Reg8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    // miscellanceous
    NOP,  // no operation
    DAA,  // decimal adjust register A
    CPL,  // complement register A (flip all bits)
    CCF,  // complement carry flag
    SCF,  // set carry flag
    HALT, // power down CPU until an interrupt occurs
    STOP, // halt CPU and LCD display until button pressed
    DI,   // disables interrupts
    EI,   // enables interrupts

    // 8-bit load instructions
    LDrr(Reg8, Reg8), // load to first register to from second register
    LDrn(Reg8),       // load value n into register. n is the next byte
    LDrHL(Reg8),      // load value stored at the address (HL) into register
    LDHLr(Reg8),      // load value stored in register to the address (HL)
}

use Instr::*;

fn read_ld_r_r(opcode: u8) -> Instr {
    let from = (opcode >> 3) & 0b111;
    let to = opcode & 0b111;
    return LDrr(Reg8::from_index(from), Reg8::from_index(to));
}

fn read_ld_r_n(opcode: u8) -> Instr {
    let to = (opcode >> 3) & 0b111;
    return LDrn(Reg8::from_index(to));
}

fn read_ld_r_hl(opcode: u8) -> Instr {
    let to = (opcode >> 3) & 0b111;
    return LDrHL(Reg8::from_index(to));
}

fn read_ld_hl_r(opcode: u8) -> Instr {
    let from = opcode & 0b111;
    return LDHLr(Reg8::from_index(from));
}

pub fn decode(opcode: u8, opcode_extra: u8) -> Instr {
    match (opcode, opcode_extra) {
        (0x00, _) => NOP,
        (0x27, _) => DAA,
        (0x2f, _) => CPL,
        (0x3f, _) => CCF,
        (0x37, _) => SCF,
        (0x76, _) => HALT,
        (0x10, 0x00) => STOP,
        (0xf3, _) => DI,
        (0xfb, _) => EI,
        (o, _) if o & 0b11000111 == 0b00000110 => read_ld_r_n(o),
        (o, _) if o & 0b11000111 == 0b01000110 => read_ld_r_hl(o),
        (o, _) if o & 0b11111000 == 0b01110000 => read_ld_hl_r(o),
        (o, _) if o & 0b11000000 == 0b01000000 => read_ld_r_r(o),
        _ => panic!("Illegal opcode {:#02x} {:#02x}", opcode, opcode_extra),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        use Reg8::*;

        assert_eq!(decode(0x00, 0x00), NOP);
        assert_eq!(decode(0x27, 0x00), DAA);
        assert_eq!(decode(0x76, 0x00), HALT);
        assert_eq!(decode(0x10, 0x00), STOP);
        assert_eq!(decode(0x41, 0x00), LDrr(B, C));
        assert_eq!(decode(0x6c, 0x00), LDrr(L, H));
        assert_eq!(decode(0x06, 0x00), LDrn(B));
        assert_eq!(decode(0x1e, 0x00), LDrn(E));
        assert_eq!(decode(0x46, 0x00), LDrHL(B));
        assert_eq!(decode(0x5e, 0x00), LDrHL(E));
        assert_eq!(decode(0x70, 0x00), LDHLr(B));
        assert_eq!(decode(0x77, 0x00), LDHLr(A));
    }
}
