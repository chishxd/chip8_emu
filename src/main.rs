struct Cpu {
    memory: [u8; 4096], //CHIP-8 can access 4KB of memory
    v: [u8; 16],        // The general purpose 16-bit registers
    i: u16,             //used to store memory addresses
    pc: u16,
    stack: [u16; 16], //used to store the address that the interpreter should return to when finished with a subroutine
    sp: u8, //The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack
    keyboard: [bool; 16],
    display: [bool; 2048],
}

impl Cpu {
    fn new() -> Self {
        Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            keyboard: [false; 16],
            display: [false; 2048],
        }
    }

    //Clears the screen, 00E0 - CLS
    fn clrscr(&mut self) {
        self.display = [false; 2048];
    }

    //Returns a value from a called subroutine and subtracts 1 from stack pointer,  00EE - RET
    fn ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack Underflow! Trying to subtract from stack pointer 0");
        }

        self.sp -= 1;

        self.pc = self.stack[self.sp as usize];
    }

    //1nnn - JP addr
    //Jump to location nnn.
    //The interpreter sets the program counter to nnn
    fn jmp(&mut self, address: u16) {
        self.pc = address;
    }

    //  The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn call(&mut self, address: u16) {
        if self.sp >= 16 {
            panic!(
                "Stack Overflow(hehe I can finally use this term)!!! 16 levels of nested subroutines"
            );
        }

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    // Skip next instruction if Vx = kk.
    //The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn se(&mut self, vx: u8, value: u8) {
        if vx == value {
            self.pc += 2;
        }
    }

    // Skip next instruction if Vx != kk.
    //The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn sne(&mut self, vx: u8, value: u8) {
        if vx != value {
            self.pc += 2;
        }
    }

    //  5xy0 - SE Vx, Vy
    // Skip next instruction if Vx = Vy.

    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn sev(&mut self, vx: u8, vy: u8) {
        if vx == vy {
            self.pc += 2;
        }
    }

    // 6xkk - LD Vx, byte
    // Set Vx = kk.

    // The interpreter puts the value kk into register Vx.
    fn load(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    // 7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.

    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn add(&mut self, x: usize, kk: u8) {
        self.v[x] += kk;
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.

    // Stores the value of register Vy in register Vx.
    fn load_v(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn tick(&mut self) {
        // This is step 1, fetching the program from 4KB ram using program counter
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[(self.pc + 1) as usize];
        let opcode = (byte1 as u16) << 8 | (byte2 as u16);

        self.pc += 2;

        // This is step 2, decoding to get the instruction nibble from opcode
        let op = ((opcode & 0xF000) >> 12) as u8; // the instruction nibble
        let x = ((opcode & 0x0F00) >> 8) as usize; //the index of general purpose index X registers
        let y = ((opcode & 0x00F0) >> 4) as usize; //the index of general purpose index y register
        let n = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;

        match op {
            0x0 => match opcode & 0x00FF {
                0xE0 => {
                    self.clrscr();
                }
                0xEE => {
                    self.ret();
                }
                _ => println!("Invalid 0x0 opcode: {:#06X}", opcode),
            },
            0x1 => self.jmp(nnn),
            0x2 => self.call(nnn),
            0x3 => {
                let vx_value = self.v[x];
                self.se(vx_value, kk);
            }
            0x4 => {
                let vx_value = self.v[x];

                self.sne(vx_value, kk);
            }
            0x5 => self.sev(self.v[x], self.v[y]),
            0x6 => self.load(x, kk),
            0x7 => self.add(x, kk),
            0x8 => match n {
                0x0 => {
                    self.load_v(x, y);
                }
                _ => println!("Invalid 0x8 series code! {:#06X}", opcode),
            },
            _ => {
                println!("Unknown opcode: {:#06X}", opcode)
            }
        }
    }
}
fn main() {
    let mut cpu = Cpu::new();
    println!("CPU initiallized at Program Counter: {:#06X} ", cpu.pc);

    cpu.tick();
}
