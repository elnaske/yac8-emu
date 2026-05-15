use sdl2::pixels::Color;

use std::default::Default;
use std::{env, path::PathBuf};

use std::collections::HashSet;

pub struct C8Config {
    pub rom_path: Option<PathBuf>,
    pub instructions_per_second: u32,
    pub pixel_size: u32,
    pub on_color: Color,
    pub off_color: Color,
    pub debug: bool,
    pub breakpoints: HashSet<u16>,
}
impl Default for C8Config {
    fn default() -> Self {
        C8Config::new(700, 10, Color::WHITE, Color::BLACK, false)
    }
}
impl C8Config {
    pub fn new(
        instructions_per_second: u32,
        pixel_size: u32,
        on_color: Color,
        off_color: Color,
        debug: bool,
    ) -> Self {
        C8Config {
            rom_path: None,
            instructions_per_second,
            pixel_size,
            on_color,
            off_color,
            debug,
            breakpoints: HashSet::new(),
        }
    }

    pub fn parse_args() -> Result<Self, String> {
        let mut cfg = Self::default();

        let args = env::args().collect::<Vec<String>>();

        if args.len() < 2 {
            panic!("Usage: yac8-emu {{path/to/rom}} {{flags}}");
        }

        let mut iterator = args.iter().skip(1).peekable();

        while let Some(arg) = iterator.next() {
            if arg.starts_with("-") {
                match arg.as_str() {
                    "--debug" => {
                        cfg.debug = true;
                        eprintln!("Emulator in DEBUG mode")
                    }
                    "--cycles" => {
                        cfg.instructions_per_second = match iterator.next() {
                            Some(arg) => Ok(arg.parse::<u32>().unwrap()),
                            None => Err("Missing argument after `--cycles`".to_string()),
                        }?;
                    }
                    "--break" => {
                        let program_start = 0x200;
                        let bytes_per_opcode = 2;

                        cfg.breakpoints = std::iter::from_fn(|| match iterator.peek() {
                            Some(arg) if !arg.starts_with("-") => iterator.next(),
                            _ => None,
                        })
                        .map(|arg| {
                            program_start
                                + bytes_per_opcode
                                    * arg.parse::<u16>().expect("Failed to parse --break arg")
                        })
                        .collect();
                    }
                    "--pixel-size" => todo!(),
                    "--on-color" => todo!(),
                    "--off-color" => todo!(),
                    _ => return Err(format!("Invalid flag: `{}`", arg)),
                }
            } else {
                match cfg.rom_path {
                    Some(p) => {
                        return Err(format!("Duplicate ROM paths: `{:?}` and `{:?}`", p, arg));
                    }
                    None => cfg.rom_path = Some(PathBuf::from(arg)),
                }
            }
        }

        Ok(cfg)
    }
}
