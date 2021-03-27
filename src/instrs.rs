use crate::gameboy::Reg8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    // 8-bit load instructions
    LDrr(Reg8, Reg8), // load to first register to from second register
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
}

use Instr::*;

fn read_ld_r_r(opcode: u8) -> Instr {
    let from = (opcode >> 3) & 0b111;
    let to = opcode & 0b111;
    println!("{} {}", from, to);
    return LDrr(Reg8::from_index(from), Reg8::from_index(to));
}

pub fn decode(opcode: u8, opcode_extra: u8) -> Instr {
    match (opcode, opcode_extra) {
        (0x00, _) => NOP,
        (0x27, _) => DAA,
        (0x2F, _) => CPL,
        (0x3F, _) => CCF,
        (0x37, _) => SCF,
        (0x76, _) => HALT,
        (0x10, 0x00) => STOP,
        (0xF3, _) => DI,
        (0xFB, _) => EI,
        (o, _) if o & 0b11000000 == 0x40 => read_ld_r_r(o),
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
        assert_eq!(decode(0x6C, 0x00), LDrr(L, H));
    }
}
