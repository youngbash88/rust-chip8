use::rand::random;

const RAM_SIZE: usize = 4096;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;


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
    pub fn pop(&mut self) -> u16{
        self.sp -= 1;
        self.stack[self.sp as usize]
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
            // NOP
            (0,0,0,0) => return,

            //Clear the screen
            (0,0,0xE,0) => {
                self.screen = [false; SCREEN_HEIGHT*SCREEN_WIDTH];
            },

            // Retrun from subroutine
            (0,0,0xE,0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },

            // Jump to address
            (1,_,_,_) => {
                let addr = op & 0x0FFF;
                self.pc = addr;
            },

            // Call subroutine
            (2,_,_,_) => {
                let addr = op & 0x0FFF;
                self.push(self.pc);
                self.pc = addr;
            },

            // Skip next instruction if Vx == nn
            (3,_,_,_) => {
                let x = digit2;
                let nn = (op & 0x00FF) as u8;
                if self.v_reg[x as usize] == nn {
                    self.pc += 2;
                }
            },
            
            // Skip next instruction if Vx != nn
            (4,_,_,_) => {
                let x = digit2;
                let nn = (op & 0x00FF) as u8;
                if self.v_reg[x as usize] != nn {
                    self.pc += 2;
                }
            },

            // Skip next instruction if Vx == Vy
            (5,_,_,0) => {
                let x = digit2;
                let y = digit3;
                if self.v_reg[x as usize] == self.v_reg[y as usize] {
                    self.pc += 2;
                }
            },

            // Load value into Vx
            (6,_,_,_) => {
                let x = digit2;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x as usize] = nn;
            },

            // Add value to Vx
            (7,_,_,_) => {
                let x = digit2;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x as usize] = self.v_reg[x as usize].wrapping_add(nn);
            },

            // Load value from Vy into Vx
            (8,_,_,0) => {
                let x = digit2;
                let y = digit3;
                self.v_reg[x as usize] = self.v_reg[y as usize];
            },

            // OR Vx with Vy
            (8,_,_,1) => {
                let x = digit2;
                let y = digit3;
                self.v_reg[x as usize] |= self.v_reg[y as usize];
            },

            // AND Vx with Vy
            (8,_,_,2) => {
                let x = digit2;
                let y = digit3;
                self.v_reg[x as usize] &= self.v_reg[y as usize];
            },

            // XOR Vx with Vy
            (8,_,_,3) => {
                let x = digit2;
                let y = digit3;
                self.v_reg[x as usize] ^= self.v_reg[y as usize];
            },

            // Add Vy to Vx
            (8,_,_,4) => {
                let x = digit2;
                let y = digit3;
                let (result, overflow) = self.v_reg[x as usize].overflowing_add(self.v_reg[y as usize]);
                self.v_reg[x as usize] = result;
                self.v_reg[0xF] = if overflow { 1 } else { 0 };
            },

            // Subtract Vy from Vx
            (8,_,_,5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (result, underflow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                self.v_reg[x] = result;
                self.v_reg[0xF] = if underflow {0} else {1};
            },

            // Shift Vx to 1 bit to the right
            (8,_,_,6) => {
                let x = digit2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            },

            // Vx = Vy - Vx
            (8,_,_,7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (result, underflow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                self.v_reg[x] = result;
                self.v_reg[0xF] = if underflow {0} else {1};
            },

            //Shift to Vx 1 but to the left
            (8,_,_,0xE) => {
                let x = digit2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },

            // Skip next instruction if Vx != Vy
            (9,_,_,0) => {
                let x = digit2;
                let y = digit3;
                if self.v_reg[x as usize] != self.v_reg[y as usize] {
                    self.pc += 2;
                }
            },

            // Load I with address
            (0xA,_,_,_) => {
                let addr = op & 0x0FFF;
                self.i_reg = addr;
            },

            // Jump to address + V0
            (0xB,_,_,_) => {
                let addr = op & 0x0FFF;
                self.pc = addr + self.v_reg[0] as u16;
            },

            // Generate random number and AND with nn
            (0xC,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                let rng:u8 = random();
                self.v_reg[x] = rng & nn;
            },

            // draw sprite
            (0xD,_,_,_) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let n = digit3;

                let x_coord = self.v_reg[x] as u16;
                let y_coord = self.v_reg[y] as u16;

                let mut flipped = false;

                for y_line in 0..n {
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        // Use mask to fetch current pixel's bit. only flip if its 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let xx = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let yy = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let index = (yy * SCREEN_WIDTH) + xx;
                            flipped |= self.screen[index];
                            self.screen[index] ^= true;
                        }
                        
                    }
                }  
            },

            // skip if key is pressed
            (0xE,_,9,0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                if self.keys[vx as usize] {
                    self.pc += 2;
                }
            },

            // skip if key is not pressed
            (0xE,_,0xA,1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                if !self.keys[vx as usize] {
                    self.pc += 2;
                }
            },

            // Load delay timer into Vx
            (0xF,_,0,7) => {
                let x = digit2 as usize;
                self.v_reg[x] = self.dt;
            },

            // Wait for key to be pressed
            (0xF,_,0,0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            },

            // Load Vx into delay timer
            (0xF,_,1,5) => {
                let x = digit2 as usize;
                self.dt = self.v_reg[x];
            },

            // Load Vx into sound timer
            (0xF,_,1,8) => {
                let x = digit2 as usize;
                self.st = self.v_reg[x];
            },

            // Add Vx to I
            (0xF,_,1,0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },

            // Set I to sprite location
            (0xF,_,2,9) => {
                let x = digit2 as usize;
                let char: u16 = self.v_reg[x] as u16;
                self.i_reg = char* 5;
            },

            // Store BCD of Vx in memory
            (0xF,_,3,3) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let i = self.i_reg as usize;

                let hundreds = vx / 100;
                let tens = (vx / 10) % 10;
                let ones = vx % 10;

                self.ram[i] = hundreds;
                self.ram[i + 1] = tens;
                self.ram[i + 2] = ones;
            },

            // Store from V0 to Vx + I registers in memory
            (0xF,_,5,5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for j in 0..=x {
                    self.ram[i + j] = self.v_reg[j];
                }
            },

            // Load registers V0 to Vx from memory
            (0xF,_,6,5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for j in 0..=x {
                    self.v_reg[j] = self.ram[i + j];
                }
            },
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

    pub fn get_screen(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress (&mut self, key: usize, pressed: bool) {
        if key < NUM_KEYS {
            self.keys[key] = pressed;
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = data.len() + START_ADDR as usize;
        self.ram[start..end].copy_from_slice(data);

    }
}