extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

const PIXEL_SIZE: u32 = 10;
const SCREEN_WIDTH: u32 = 64 * PIXEL_SIZE;
const SCREEN_HEIGHT: u32 = 32 * PIXEL_SIZE;

pub struct Display {
    pub canvas: WindowCanvas,
}
impl Display {
    pub fn new(window: Window) -> Result<Self, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Display { canvas })
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.canvas.fill_rect(Rect::new(
            x * PIXEL_SIZE as i32,
            y * PIXEL_SIZE as i32,
            PIXEL_SIZE,
            PIXEL_SIZE,
        ))?;
        Ok(())
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("test", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut display = Display::new(window).unwrap();

    for x in 0..64 {
        for y in 0..32 {
            let color = {
                if (x % 2 == 0) ^ (y % 2 == 0) {
                    Color::BLACK
                } else {
                    Color::WHITE
                }
            };
            display.canvas.set_draw_color(color);
            display.draw_pixel(x, y).unwrap();
        }
    }
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
