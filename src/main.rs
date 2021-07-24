mod cpu_funcs;
mod gameboy;
mod instrs;
mod special_registers;

use gameboy::GameBoy;

fn main() {
    let gb = &mut GameBoy::new();
}
