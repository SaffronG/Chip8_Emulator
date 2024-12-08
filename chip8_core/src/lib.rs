pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
pub struct NUM_KEYS: usize = 16;

pub struct Emu {
    pc: u16, // Program Counter
    ram: [u8; RAM_SIZE], // Arr of u8 to be chip8's RAM
    screen: [bool, SCREEN_WIDTH * SCREEN_HEIGHT], // Arr of Bool
    v_reg: [u8; NUM_REGS], // Overflow Registers
    i_reg: u16, // Index Register (singular for R/W cmds)
    sp: u16; // Stack Pointer
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS], // Stores key presses
    dt: u8, // Delay Timer
    st: u8, // Sound Timer
}
