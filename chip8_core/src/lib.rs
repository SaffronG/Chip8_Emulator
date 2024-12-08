pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
pub struct NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;

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

impl Emu {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false, SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0, STACK_SIZE],
            keys: [false, NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }

    fn push(&mut self, val: u16) -> u16 {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    fn pop(&mut self val u16) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
