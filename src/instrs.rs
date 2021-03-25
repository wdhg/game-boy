#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
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
        _ => NOP,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode(0x00, 0x00), NOP);
        assert_eq!(decode(0x27, 0x00), DAA);
        assert_eq!(decode(0x76, 0x00), HALT);
        assert_eq!(decode(0x10, 0x00), STOP);
    }
}
