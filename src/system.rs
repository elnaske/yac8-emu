use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::default::Default;
use std::path::PathBuf;

use crate::display::C8Display;
use crate::instructions::Instruction;
use crate::input::get_input;

const FONT_DATA: [u8; 80] = [
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
];

pub struct C8Config {
   pub pixel_size: u32,
   pub on_color: Color,
   pub off_color: Color,
   pub debug: bool,
}
impl Default for C8Config {
    fn default() -> Self {
        C8Config::new(10, Color::WHITE, Color::BLACK, false)
    }
}
impl C8Config {
    pub fn new(pixel_size: u32, on_color: Color, off_color: Color, debug: bool) -> Self {
        C8Config { pixel_size, on_color, off_color, debug }
    }
}

pub struct Chip8 {
    memory: [u8; 4096],
    display: C8Display,
    var_regs: [u8; 16],
    stack: Vec<u16>,
    pc: u16,
    idx_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
}
impl Chip8 {
    pub fn new(
        window: Window,
        on_color: Color,
        off_color: Color,
        debug: bool,
    ) -> Result<Self, String> {
        let display = C8Display::new(window, on_color, off_color, debug)?;

        let mut state = Chip8 {
            memory: [0; 4096],
            display,
            var_regs: [0; 16],
            stack: Vec::new(),
            pc: 0x200,
            idx_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
        };

        state.memory[0x50..=0x09F].copy_from_slice(&FONT_DATA);

        // TODO: load rom

        Ok(state)
    }

    pub fn from_config(window: Window, cfg: C8Config) -> Result<Self, String> {
        Chip8::new(window, cfg.on_color, cfg.off_color, cfg.debug)
    }

    fn load_rom(&mut self, path: PathBuf) -> Result<(), std::io::Error> {
        todo!()
    }

    pub fn run(&mut self, sdl_context: Sdl) -> Result<(), String> {
        self.display.draw_screen_buffer()?;
        self.display.canvas.present();
    
        let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        if let Some(key) = get_input(keycode) {
                            self.keypad[key as usize] = true;
                            eprintln!("Keys pressed: {:?}", self.keypad);
                        }
                    }
                    Event::KeyUp {keycode: Some(keycode), ..} => {
                        if let Some(key) = get_input(keycode) {
                            self.keypad[key as usize] = false;
                            eprintln!("Keys pressed: {:?}", self.keypad);
                        }
                    }
                    _ => {
                        let op_code = self.fetch_opcode();

                        // TODO: decode instruction
                        // TODO: execute instruction
                        // TODO: update timers
                        // TODO: sleep
                    }
                }
            }
        }

        Ok(())
    }

    fn fetch_opcode(&mut self) -> Option<u16> {
        let upper = *self.memory.get(self.pc as usize)? as u16;
        let lower = *self.memory.get((self.pc + 1) as usize)? as u16;

        self.pc += 2;

        Some((upper << 8) | lower)
    }


}
