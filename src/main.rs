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

    fn tick(&mut self) {
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[(self.pc + 1) as usize];
        let opcode = (byte1 as u16) << 8 | (byte2 as u16);

        self.pc += 2;

        let first_nibble = (opcode & 0xF000) >> 12;

        match first_nibble {
            0x0 => match opcode & 0x00FF {
                0xE0 => {
                    self.clrscr();
                }
                0xEE => {
                    self.ret();
                }
                _ => println!("Invalid 0x0 opcode: {:#06X}", opcode),
            },
            0x1 => {
                let address = opcode & 0x00FF;
                self.jmp(address);
            }
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
