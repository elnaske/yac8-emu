use egui_sdl2::egui::{self, Align2};
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;

use rand::{Rng, RngExt};

use std::collections::HashSet;
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
    breakpoints: HashSet<u16>,
    reset: bool,
}
impl Chip8 {
    pub fn new(
        window: Window,
        on_color: Color,
        off_color: Color,
        instructions_per_second: u32,
        breakpoints: HashSet<u16>,
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
            breakpoints,
            reset: false,
        };

        state.memory[0x50..=0x09F].copy_from_slice(&FONT_DATA);

        Ok(state)
    }

    pub fn from_config(window: Window, cfg: C8Config) -> Result<Self, String> {
        Chip8::new(
            window,
            cfg.on_color,
            cfg.off_color,
            cfg.instructions_per_second,
            cfg.breakpoints,
            cfg.debug,
        )
    }

    fn load_rom(&mut self, path: &PathBuf) -> Result<(), std::io::Error> {
        let mut file = File::open(path)?;
        file.read(&mut self.memory[0x200..])?; // TODO: read_exact doesn't work on all systems
        Ok(())
    }

    fn reset_state(&mut self) {
        // TODO: rewrite and implement with new()
        self.memory = [0; 4096];
        self.memory[0x50..=0x09F].copy_from_slice(&FONT_DATA);
        self.var_regs = [0; 16];
        self.idx_reg = 0;
        self.stack = Vec::new();
        self.pc = 0x200;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad = [false; 16];
        self.display.buff.fill(false);
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

            if self.breakpoints.contains(&self.pc) {
                self.paused = true;
                self.breakpoints.remove(&self.pc);
            }

            for event in event_pump.poll_iter() {
                let _ = self.display.canvas.on_event(&event);

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
                        // keypad
                        if let Some(key) = get_input(keycode) {
                            self.keypad[key as usize] = true;
                            eprintln!("Keys pressed: {:?}", self.keypad);
                        }

                        // pause toggle
                        if keycode == Keycode::Space {
                            self.paused ^= true;
                        }

                        // manual advance
                        if self.paused && keycode == Keycode::N {
                            if let Err(e) = self.run_next_instruction() {
                                eprint!("{}", e);
                                break 'running;
                            }
                        }

                        // debug toggle
                        if keycode == Keycode::Tab {
                            self.debug ^= true;
                            self.display.debug_lines ^= true;
                            self.display.canvas.clear([0, 0, 0, 255]);
                            self.display.draw_screen_buffer()?;
                        }

                        // reset
                        if keycode == Keycode::Backspace {
                            self.reset = true;
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => {
                        // keypad release
                        if let Some(key) = get_input(keycode) {
                            self.keypad[key as usize] = false;
                            eprintln!("Keys pressed: {:?}", self.keypad);
                        }
                    }
                    _ => {}
                }
            }

            if self.debug {
                self.show_debug_ui();
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

            if self.reset {
                self.reset_state();
                self.load_rom(&rom_path).map_err(|e| e.to_string())?;

                self.display.canvas.clear([0, 0, 0, 255]);
                self.display.draw_screen_buffer()?;

                self.paused = true;
                self.reset = false;
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
            (0x0, 0x0, 0xE, 0xE) => {
                // 00EE - Return from subroutine
                self.pc = self.stack.pop().expect("No subroutine to return from");
            }
            (0x1, _, _, _) => {
                // 1NNN - Jump
                self.pc = op_code & 0x0FFF;
            }
            (0x2, _, _, _) => {
                // 2NNN - Call subroutine
                self.stack.push(self.pc);
                self.pc = op_code & 0x0FFF;
            }
            (0x3, x, _, _) => {
                // 3XNN - Skip if Vx == NN
                let val = (op_code & 0x00FF) as u8;
                let vx = self.var_regs[x as usize];
                if vx == val {
                    self.pc += 2;
                }
            }
            (0x4, x, _, _) => {
                // 4XNN - Skip if Vx != NN
                let val = (op_code & 0x00FF) as u8;
                let vx = self.var_regs[x as usize];
                if vx != val {
                    self.pc += 2;
                }
            }
            (0x5, x, y, 0x0) => {
                // 5XY0 - Skip if Vx == Vy
                let vx = self.var_regs[x as usize];
                let vy = self.var_regs[y as usize];
                if vx == vy {
                    self.pc += 2;
                }
            }
            (0x6, x, _, _) => {
                // 6XNN - Set Register
                self.var_regs[x as usize] = (op_code & 0x00FF) as u8;
            }
            (0x7, x, _, _) => {
                // 7XNN - Add to Register
                self.var_regs[x as usize] += (op_code & 0x00FF) as u8;
            }
            (0x8, x, y, op) => {
                // 8XY0-E - Math / Bitwise / Assignment Operations
                let mut flag = self.var_regs[0xF];
                let vy = self.var_regs[y as usize];
                let vx = &mut self.var_regs[x as usize];

                match op {
                    0x0 => *vx = vy,
                    0x1 => *vx |= vy,
                    0x2 => *vx &= vy,
                    0x3 => *vx ^= vy,
                    0x4 => {
                        flag = if *vx > u8::MAX - vy { 1 } else { 0 };
                        *vx += vy;
                    }
                    0x5 => {
                        flag = if *vx >= vy { 1 } else { 0 };
                        *vx -= vy;
                    }
                    0x6 => {
                        flag = *vx & 0x0001;
                        *vx >>= 1;
                    }
                    0x7 => {
                        flag = if vy >= *vx { 1 } else { 0 };
                        *vx = vy - *vx;
                    }
                    0xE => {
                        flag = (*vx & 0x80) >> 7;
                        *vx <<= 1;
                    }
                    _ => (),
                };

                self.var_regs[0xF] = flag;
            }
            (0x9, x, y, 0x0) => {
                // 9XY0 - Skip if Vx != Vy
                let vx = self.var_regs[x as usize];
                let vy = self.var_regs[y as usize];
                if vx != vy {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                // ANNN - Set Index Register
                self.idx_reg = op_code & 0x0FFF;
            }
            (0xB, _, _, _) => {
                // BNNN - Jump to NNN + V0
                self.pc = self.var_regs[0x0] as u16 + (op_code & 0x0FFF);
            }
            (0xC, x, _, _) => {
                // CXNN - Random Number
                let r = rand::rng().random::<u8>();
                self.var_regs[x as usize] = r & ((op_code & 0x00FF) as u8);
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
            (0xE, x, 0x9, 0xE) => {
                // EX9E - Skip if key pressed
                // TODO
            }
            (0xE, x, 0xA, 0x1) => {
                // EXA1 - Skip if key not pressed
                // TODO
            }
            (0xF, x, 0x0, 0x7) => {
                // FX07 - Get delay timer
                // TODO
            }
            (0xF, x, 0x0, 0xA) => {
                // FX0A - Await key press
                // TODO
            }
            (0xF, x, 0x1, 0x5) => {
                // FX15 - Set delay timer
                // TODO
            }
            (0xF, x, 0x1, 0x8) => {
                // FX18 - Set sound timer
                // TODO
            }
            (0xF, x, 0x1, 0xE) => {
                // FX1E - Add to I
                self.idx_reg += self.var_regs[x as usize] as u16;
            }
            (0xF, x, 0x2, 0x9) => {
                // FX29 - Set I to sprite address
                // TODO
            }
            (0xF, x, 0x3, 0x3) => {
                // FX33 - Binary-coded decimal
                let vx = self.var_regs[x as usize];
                let idx = self.idx_reg as usize;

                self.memory[idx] = vx / 100;
                self.memory[idx + 1] = (vx / 10) % 10;
                self.memory[idx + 2] = vx % 10;
            }
            (0xF, x, 0x5, 0x5) => {
                // FX55 - Dump registers V0 - Vx to memory
                let start = self.idx_reg as usize;
                let end = start + x as usize;
                self.memory[start..=end].copy_from_slice(&self.var_regs[0..=x as usize]);
            }
            (0xF, x, 0x6, 0x5) => {
                // FX65 - Load registers V0 - Vx from memory
                let start = self.idx_reg as usize;
                let end = start + x as usize;
                self.var_regs[0..=x as usize].copy_from_slice(&self.memory[start..=end]);
            }
            _ => eprintln!("Unknown Instruction: {:#x}", op_code),
        }
        Ok(())
    }

    // TODO: move into display.rs
    fn show_debug_ui(&mut self) {
        self.display.canvas.run(|ctx| {
            egui::Window::new("Program State").show(ctx, |ui| {
                if ui.add(egui::Button::new("Reset ROM")).clicked() {
                    self.reset = true;
                }
                ui.separator();
                ui.label("Variable Registers:");
                for (reg, val) in self.var_regs.iter().enumerate() {
                    ui.label(format!("\t{:x}: {:#x}", reg, val));
                }
                ui.separator();
                ui.label(format!("I: {:#x}", self.idx_reg));
                ui.label(format!("PC: {:#x}", self.pc));
                ui.label(format!("Delay Timer: {:#x}", self.delay_timer));
                ui.label(format!("Sound Timer: {:#x}", self.sound_timer));
                ui.separator();
                ui.label(format!("Stack: {:#x?}", self.stack));
            });

            egui::Window::new("Debug Information").show(ctx, |ui| {
                ui.add(
                    egui::Slider::new(&mut self.instructions_per_second, 1..=1000)
                        .text("Instructions_per_second"),
                );
                ui.label(format!("Paused: {}", self.paused));
                ui.label("Breakpoints:");
                for breakpoint in self.breakpoints.iter() {
                    ui.label(format!("\t{breakpoint}"));
                }
            });

            egui::Window::new("Memory")
                .vscroll(true)
                .anchor(Align2::RIGHT_TOP, [-15.0, 15.0])
                .show(ctx, |ui| {
                    // TODO: alignment
                    ui.label(format!("{:x?}", self.memory));
                });
        });
        self.display.canvas.paint();
        self.display.canvas.present();
    }
}
