mod cpu;

use std::{fs::File, io::Read};

use cpu::Cpu;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut cpu = Cpu::new();

    let mut file =
        File::open("/home/chish/Downloads/pumpkindressup.ch8").expect("failed to open ROM");
    let mut rom_buffer = Vec::new();
    file.read_to_end(&mut rom_buffer)
        .expect("failed to read ROM");

    for (i, &byte) in rom_buffer.iter().enumerate() {
        cpu.memory[0x200 + i] = byte;
    }

    let mut window = Window::new(
        "The Chip-8 Emu",
        64,
        32,
        WindowOptions {
            scale: minifb::Scale::X16, //this will make actual window size 512 * 256
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));

    window.set_target_fps(60);

    let mut pixel_buf: [u32; 2048] = [0; 2048];
    while window.is_open() & !window.is_key_down(Key::Escape) {
        update_keys(&window, &mut cpu);

        for _ in 0..8 {
            cpu.tick();
        }

        if cpu.delay_timer > 0 {
            cpu.delay_timer -= 1;
        }

        if cpu.sound_timer > 0 {
            cpu.sound_timer -= 1;
        }

        for (pixel, &is_on) in pixel_buf.iter_mut().zip(cpu.display.iter()) {
            *pixel = if is_on { 0x00FFFFFF } else { 0x00000000 };
        }

        window.update_with_buffer(&pixel_buf, 64, 32).unwrap();
    }
}

fn update_keys(window: &Window, cpu: &mut Cpu) {
    cpu.keyboard[0x1] = window.is_key_down(Key::Key1);
    cpu.keyboard[0x2] = window.is_key_down(Key::Key2);
    cpu.keyboard[0x3] = window.is_key_down(Key::Key3);
    cpu.keyboard[0xC] = window.is_key_down(Key::Key4);

    //Row 2
    cpu.keyboard[0x4] = window.is_key_down(Key::Q);
    cpu.keyboard[0x5] = window.is_key_down(Key::W);
    cpu.keyboard[0x6] = window.is_key_down(Key::E);
    cpu.keyboard[0xD] = window.is_key_down(Key::R);

    //Row 3
    cpu.keyboard[0x7] = window.is_key_down(Key::A);
    cpu.keyboard[0x8] = window.is_key_down(Key::S);
    cpu.keyboard[0x9] = window.is_key_down(Key::D);
    cpu.keyboard[0xE] = window.is_key_down(Key::F);

    //Row 2
    cpu.keyboard[0xA] = window.is_key_down(Key::Z);
    cpu.keyboard[0x0] = window.is_key_down(Key::X);
    cpu.keyboard[0xB] = window.is_key_down(Key::C);
    cpu.keyboard[0xF] = window.is_key_down(Key::V);
}

// https://www.youtube.com/watch?v=QDia3e12czc&pp=ygUIcmlja3JvbGw%3D
