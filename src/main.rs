extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub mod display;
use display::{C8Display, SCREEN_HEIGHT, SCREEN_WIDTH, ScreenBuffer};

pub mod system;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("yac8-emu", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut display =
        C8Display::new(window, Color::WHITE, Color::BLACK, true).expect("Failed to create display");
    let screen = ScreenBuffer::new();

    // let mut screen = ScreenBuffer::new();
    // for x in 0..64 {
    //     for y in 0..32 {
    //         screen[y][x] = (x % 2 == 0) ^ (y % 2 == 0);
    //     }
    // }

    display
        .draw_screen_buffer(screen)
        .expect("Failed to draw on screen");
    display.canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    // main logic goes here
                }
            }
        }
    }

    // canvas.clear();
    // canvas.present();
}
