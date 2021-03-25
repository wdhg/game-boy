#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    NOP,
    DAA,
    HALT,
}

use Instr::*;

pub fn decode(opcode: u8) -> Instr {
    match opcode {
        0x00 => NOP,
        0x27 => DAA,
        0x76 => HALT,
        _ => NOP,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode(0x00), NOP);
        assert_eq!(decode(0x27), DAA);
        assert_eq!(decode(0x76), HALT);
    }
}
