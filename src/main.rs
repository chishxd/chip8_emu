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
    fn se(&mut self, x: usize, value: u8) {
        if self.v[x] == value {
            self.pc += 2;
        }
    }

    // Skip next instruction if Vx != kk.
    //The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn sne(&mut self, x: usize, value: u8) {
        if self.v[x] != value {
            self.pc += 2;
        }
    }

    //  5xy0 - SE Vx, Vy
    // Skip next instruction if Vx = Vy.

    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn sev(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
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

    // 8xy1 - OR Vx, Vy
    // Set Vx = Vx OR Vy.

    // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    //  A bitwise OR compares the corrseponding bits from two values,
    // and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn or(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx AND Vy.

    // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    // A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
    // then the same bit in the result is also 1. Otherwise, it is 0.
    fn and(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    // 8xy3 - XOR Vx, Vy
    // Set Vx = Vx XOR Vy.

    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    // An exclusive OR compares the corrseponding bits from two values,
    // and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn xor(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    // 8xy4 - ADD Vx, Vy
    // Set Vx = Vx + Vy, set VF = carry.

    // The values of Vx and Vy are added together.
    // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    // Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn add_v(&mut self, x: usize, y: usize) {
        self.v[x] += self.v[y];
    }

    // 8xy5 - SUB Vx, Vy
    // Set Vx = Vx - Vy, set VF = NOT borrow.

    // If Vx > Vy, then VF is set to 1, otherwise 0.
    //  Then Vy is subtracted from Vx, and the results stored in Vx.
    fn sub_v(&mut self, x: usize, y: usize) {
        let borrow_flag = if self.v[x] >= self.v[y] { 1 } else { 0 };

        self.v[x] = self.v[x].wrapping_sub(self.v[y]);

        self.v[0xF] = borrow_flag;
    }

    // 8xy6 - SHR Vx {, Vy}
    // Set Vx = Vx SHR 1.

    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    // Then Vx is divided by 2.
    fn shr(&mut self, x: usize) {
        let lsb = self.v[x] & 1;
        self.v[x] >>= 1;

        self.v[0xF] = lsb;
    }

    // 8xy7 - SUBN Vx, Vy
    // Set Vx = Vy - Vx, set VF = NOT borrow.

    // If Vy > Vx, then VF is set to 1, otherwise 0.
    //  Then Vx is subtracted from Vy, and the results stored in Vx
    fn subn(&mut self, x: usize, y: usize) {
        let borrow_flag = if self.v[y] >= self.v[x] { 1 } else { 0 };

        self.v[x] = self.v[y].wrapping_sub(self.v[x]);

        self.v[0xF] = borrow_flag;
    }

    // 8xyE - SHL Vx {, Vy}
    // Set Vx = Vx SHL 1.

    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    //  Then Vx is multiplied by 2.
    fn shl(&mut self, x: usize) {
        let msb = (self.v[x] >> 7) & 1;

        self.v[x] <<= 1;

        self.v[0xF] = msb;
    }

    // 9xy0 - SNE Vx, Vy
    // Skip next instruction if Vx != Vy.

    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn sne_v(&mut self, vx: u8, vy: u8) {
        if vx != vy {
            self.pc += 2;
        }
    }
    //  Annn - LD I, addr
    // Set I = nnn.
    //
    // The value of register I is set to nnn.
    //
    fn load_addr(&mut self, nnn: u16) {
        self.i = nnn;
    }

    //     Bnnn - JP V0, addr
    // Jump to location nnn + V0.
    //
    // The program counter is set to nnn plus the value of V0.
    //
    fn jmp_v0(&mut self, nnn: u16) {
        let target = nnn + (self.v[0] as u16);
        self.pc = target & 0x0FFF;
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
            0x3 => self.se(x, kk),
            0x4 => self.sne(x, kk),
            0x5 => self.sev(x, y),
            0x6 => self.load(x, kk),
            0x7 => self.add(x, kk),
            0x8 => match n {
                0x0 => self.load_v(x, y),
                0x1 => self.or(x, y),
                0x2 => self.and(x, y),
                0x3 => self.xor(x, y),
                0x4 => self.add_v(x, y),
                0x5 => self.sub_v(x, y),
                0x6 => self.shr(x),
                0x7 => self.subn(x, y),
                0xE => self.shl(x),

                _ => println!("Invalid 0x8 series code! {:#06X}", opcode),
            },
            0x9 => self.sne_v(self.v[x], self.v[y]),
            0xA => self.load_addr(nnn),
            0xB => self.jmp_v0(nnn),
            _ => {
                println!("Unknown opcode: {:#06X}", opcode)
            }
        }
    }
}
fn main() {
    let mut cpu = Cpu::new();

    //[YEAH I KNOW TESTING HERE IS SLOPPY... I will fix it soon :)]

    // loading up some values like 10 and 20
    cpu.memory[0x200] = 0x61;
    cpu.memory[0x201] = 0x0A; //512: LD V1, 10 (0x610A)

    cpu.memory[0x202] = 0x62;
    cpu.memory[0x203] = 0x14; // 514: LD V2, 20  (0x6214)

    // calling a subroutine(subroutine is defined below)
    cpu.memory[0x204] = 0x22;
    cpu.memory[0x205] = 0x0A; // 516: CALL 0x20A (0x220A)

    cpu.memory[0x206] = 0x12;
    cpu.memory[0x207] = 0x00; // 518: JP 0x200   (0x1200)

    // Now time for subroutines!
    // this jst loads value from v2 to v1
    cpu.memory[0x20A] = 0x81;
    cpu.memory[0x20B] = 0x20; // 522: LD V1, V2 (0x8120)
    cpu.memory[0x20C] = 0x00;
    cpu.memory[0x20D] = 0xEE; //RET (0x00EE)

    for step in 1..=6 {
        println!("\n===TICK {}===", step);
        cpu.tick();

        println!("Program Counter (PC): {:#06X}", cpu.pc);
        println!("Stack Pointer(SP): {}", cpu.sp);
        println!("Register V1: {}", cpu.v[1]);
        println!("Register V2: {}", cpu.v[2]);
        println!("Register V3: {}", cpu.v[3]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jmp() {
        let mut cpu = Cpu::new();

        cpu.jmp(0x300);

        assert_eq!(cpu.pc, 0x300);
    }

    #[test]
    fn test_se_skips_when_equal() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x200;

        cpu.se(1, 0);

        //The program should have incremented by 2 now
        assert_eq!(cpu.pc, 0x202);
    }

    //TODO: Write up all Tests... maybe in a different file.
}
