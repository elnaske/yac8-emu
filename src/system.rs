use std::ops::{Deref, DerefMut};

use sdl2::pixels::Color;
use sdl2::video::Window;

use crate::display::C8Display;

struct ProgramCounter(usize);
impl Deref for ProgramCounter {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ProgramCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn get_font_data() -> [u8; 80] {
    [
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
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ]
}

pub struct Chip8 {
    memory: [u8; 4096],
    display: C8Display,
    var_regs: [u8; 16],
    stack: Vec<u16>,
    pc: ProgramCounter,
    idx_reg: u16,
    delay_timer: u8, // TODO: type
    sound_timer: u8, // TODO: type
}
impl Chip8 {
    pub fn new(
        window: Window,
        on_color: Color,
        off_color: Color,
        debug: bool,
    ) -> Result<Self, String> {

        let display = C8Display::new(window, on_color, off_color, debug)?;
        Ok(Chip8 {
            memory: Chip8::setup_memory(),
            display,
            var_regs: [0; 16],
            stack: Vec::new(),
            pc: ProgramCounter(0),
            idx_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
        })
    }

    fn setup_memory() -> [u8; 4096] {
        let mut memory = [0; 4096];

        // Store font from 0x050 to 0x09F
        let font = get_font_data();
        let font_start_byte =  0x50;
        
        for i in 0..font.len(){
            memory[font_start_byte + i] = font[i];
        }

        memory
    }
}
