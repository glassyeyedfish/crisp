const COLS: usize = 664;
const ROWS: usize = 32;
const V_SIZE: usize = 16;
const STACK_SIZE: usize = 512;
const MEM_SIZE: usize = 4096;

pub struct Chip8 {
    i: u16,
    dt: u8,
    st: u8,
    v: [u8; V_SIZE],
    stack: [u8; STACK_SIZE],
    sp: usize,
    mem: [u8; MEM_SIZE],
    pc: usize,
    display: [u8; COLS * ROWS],
}
