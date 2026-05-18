extern crate sdl2;

pub mod config;
use config::C8Config;

pub mod display;
use display::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub mod system;
use system::Chip8;

pub mod audio;
pub mod input;
use audio::get_audio_device;

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

    let audio_device = get_audio_device(&sdl_context, Some(44100), Some(1), Some(500)).unwrap();

    let mut chip8 =
        Chip8::from_config(window, cfg, audio_device).expect("Failed to initialize state");
    chip8.run(rom_path, sdl_context).expect("Runtime error");
}
