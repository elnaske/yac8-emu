extern crate sdl2;

pub mod display;
use display::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub mod system;
use system::{C8Config, Chip8};

pub mod instructions;
pub mod input;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("yac8-emu", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let cfg = {
        let mut cfg = C8Config::default();
        cfg.debug = true;
        cfg
    };

    let mut chip8 = Chip8::from_config(window, cfg).expect("Failed to initialize state");
    chip8.run(sdl_context).expect("Runtime error");
}
