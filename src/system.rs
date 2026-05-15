use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::config::C8Config;
use crate::display::C8Display;
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
    instructions_per_second: u32,
    debug: bool,
    paused: bool,
}
impl Chip8 {
    pub fn new(
        window: Window,
        on_color: Color,
        off_color: Color,
        instructions_per_second: u32,
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
            instructions_per_second,
            debug,
            paused: false,
        };

        state.memory[0x50..=0x09F].copy_from_slice(&FONT_DATA);

        Ok(state)
    }

    pub fn from_config(window: Window, cfg: &C8Config) -> Result<Self, String> {
        Chip8::new(
            window,
            cfg.on_color,
            cfg.off_color,
            cfg.instructions_per_second,
            cfg.debug,
        )
    }

    fn load_rom(&mut self, path: &PathBuf) -> Result<(), std::io::Error> {
        let mut file = File::open(path)?;
        file.read(&mut self.memory[0x200..])?; // TODO: read_exact doesn't work on all systems
        Ok(())
    }

    pub fn run(&mut self, rom_path: PathBuf, sdl_context: Sdl) -> Result<(), String> {
        self.load_rom(&rom_path).map_err(|e| e.to_string())?;
        if self.debug {
            eprintln!("Loaded ROM: `{:?}`", rom_path); // TODO: absolute path
        }

        self.display.draw_screen_buffer()?;
        self.display.canvas.present();

        let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        'running: loop {
            let start_time = Instant::now();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        if let Some(key) = get_input(keycode) {
                            self.keypad[key as usize] = true;
                            eprintln!("Keys pressed: {:?}", self.keypad);
                        }
                        if keycode == Keycode::Space {
                            self.paused ^= true;
                        }
                        if self.paused && keycode == Keycode::N {
                            if let Err(e) = self.run_next_instruction() {
                                eprint!("{}", e);
                                break 'running;
                            }
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => {
                        if let Some(key) = get_input(keycode) {
                            self.keypad[key as usize] = false;
                            eprintln!("Keys pressed: {:?}", self.keypad);
                        }
                    }
                    _ => {}
                }
            }

            if !self.paused {
                if let Err(e) = self.run_next_instruction() {
                    eprint!("{}", e);
                    break 'running;
                }

                let elapsed = start_time.elapsed();

                let instruction_delay_msec: Duration =
                    Duration::from_millis(1000 / self.instructions_per_second as u64);
                if elapsed < instruction_delay_msec {
                    std::thread::sleep(instruction_delay_msec - elapsed);
                }
            }
        }

        Ok(())
    }

    fn run_next_instruction(&mut self) -> Result<(), String> {
        let op_code = self.fetch_opcode();

        if self.debug
            && let Some(oc) = op_code
        {
            eprintln!("Opcode: {:#x}", oc);
        }

        match op_code {
            None => Err("Reached end of file; Terminating".to_string()),
            Some(op) => {
                self.execute_opcode(op)?;

                if self.delay_timer > 0 {
                    self.delay_timer -= 1;
                }
                if self.sound_timer > 0 {
                    self.sound_timer -= 1;
                }

                Ok(())
            }
        }
    }

    fn fetch_opcode(&mut self) -> Option<u16> {
        let upper = *self.memory.get(self.pc as usize)? as u16;
        let lower = *self.memory.get((self.pc + 1) as usize)? as u16;

        self.pc += 2;

        Some((upper << 8) | lower)
    }

    fn execute_opcode(&mut self, op_code: u16) -> Result<(), String> {
        let nib1 = (op_code & 0xF000) >> 12;
        let nib2 = (op_code & 0x0F00) >> 8;
        let nib3 = (op_code & 0x00F0) >> 4;
        let nib4 = op_code & 0x000F;

        match (nib1, nib2, nib3, nib4) {
            (0x0, 0x0, 0xE, 0x0) => {
                // 00E0 - Clear Screen
                self.display.buff.fill(false);
                self.display.draw_screen_buffer()?;
            }
            (0x1, _, _, _) => {
                // 1NNN - Jump
                self.pc = op_code & 0x0FFF;
            }
            (0x6, x, _, _) => {
                // 6XNN - Set Register
                self.var_regs[x as usize] = (op_code & 0x0FF) as u8;
            }
            (0x7, x, _, _) => {
                // 7XNN - Add to Register
                self.var_regs[x as usize] += (op_code & 0x0FF) as u8;
            }
            (0xA, _, _, _) => {
                // ANNN - Set Index Register
                self.idx_reg = op_code & 0x0FFF;
            }
            (0xD, x, y, height) => {
                // DXYN - Draw to Screen
                let screen_x = self.var_regs[x as usize] as usize;
                let screen_y = self.var_regs[y as usize] as usize;

                self.var_regs[0xF] = 0;

                for row in 0..height as usize {
                    // stop at bottom edge of screen
                    if screen_y + row >= 32 {
                        break;
                    }

                    let row_byte = self.memory[self.idx_reg as usize + row];
                    for col in 0..8_usize {
                        // stop at right edge of screen
                        if screen_x + col >= 64 {
                            break;
                        }
                        if row_byte & (0x80 >> col) > 0 {
                            let pixel_idx = (screen_y + row) * 64 + (screen_x + col);
                            if self.display.buff[pixel_idx] {
                                self.var_regs[0xF] = 1;
                            }
                            self.display.buff[pixel_idx] ^= true;
                        }
                    }
                }

                self.display.draw_screen_buffer()?;
            }
            _ => todo!("Remaining instructions"),
        }
        Ok(())
    }
}
