const RAM_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;


const NUM_REG: usize = 16;
const NUM_KEYS: usize = 16;
const STACK_SIZE: usize = 16;


pub struct Emu {
    pc: u16,
    ram: [u8;RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REG],
    i_reg: u16,
    keys: [bool; NUM_KEYS],

    stack: [u16; STACK_SIZE],
    sp: u16,    

    dt: u8,
    st: u8,
}

const START_ADDR: u16 = 0x200;

const FONT_SIZE: usize = 80;
// Load fontset
const FONTSET: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];
impl Emu {
    pub fn new() -> Self{
        let mut emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REG],
            i_reg: 0,
            keys: [false; NUM_KEYS],
            stack: [0; STACK_SIZE],
            sp: 0,
            dt: 0,
            st: 0,
        };
        emu.ram[..FONT_SIZE].copy_from_slice(&FONTSET);
        emu

    }
    
    // these are for the stack
    pub fn push(&mut self, val: u16){
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    pub fn pop(&mut self) {
        self.sp -= 1;
        self.stack[self.sp as usize];
    }

    pub fn reset(&mut self){
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REG];
        self.i_reg = 0;
        self.keys = [false; NUM_KEYS];
        self.stack = [0; STACK_SIZE];
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.ram[..FONT_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self){
        //fetch
        let op = self.fetch();

        //decode & execute
        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte= self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }

    pub fn tick_timer(&mut self){
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                // sound buzzer
            }
            self.st -= 1;
        }
    }

    fn execute(&mut self, op: u16){
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = (op & 0x000F);


        match (digit1, digit2, digit3, digit4) {
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
}