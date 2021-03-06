use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(PartialEq, Eq, Hash)]
enum Register16 {
    SP,
    PC,
}

struct GameBoy {
    registers8: HashMap<Register8, u8>,
    registers16: HashMap<Register16, u16>,
}

impl GameBoy {
    fn new() -> GameBoy {
        let mut registers8 = HashMap::new();
        registers8.insert(Register8::A, 0);
        registers8.insert(Register8::B, 0);
        registers8.insert(Register8::C, 0);
        registers8.insert(Register8::D, 0);
        registers8.insert(Register8::E, 0);
        registers8.insert(Register8::F, 0);
        registers8.insert(Register8::H, 0);
        registers8.insert(Register8::L, 0);

        let mut registers16 = HashMap::new();
        registers16.insert(Register16::PC, 0x100);
        registers16.insert(Register16::SP, 0xfff);

        return GameBoy {
            registers8: registers8,
            registers16: registers16,
        };
    }
}

fn main() {
    let gb = GameBoy::new();
}
