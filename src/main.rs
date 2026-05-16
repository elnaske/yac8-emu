extern crate sdl2;

pub mod config;
use config::C8Config;

pub mod display;
use display::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub mod system;
use system::Chip8;

pub mod input;

fn main() {
    let cfg = C8Config::parse_args().expect("Error parsing arguments");

    let rom_path = match cfg.rom_path {
        Some(ref path) => path.clone(),
        None => panic!("Invalid ROM path"),
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("yac8-emu", SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2) // TODO: hacky
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut chip8 = Chip8::from_config(window, cfg).expect("Failed to initialize state");
    chip8.run(rom_path, sdl_context).expect("Runtime error");
}
