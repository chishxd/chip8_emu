mod cpu;

use cpu::Cpu;

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
