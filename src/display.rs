use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use std::ops::{Deref, DerefMut};

const PIXEL_SIZE: u32 = 11;
pub const SCREEN_WIDTH: u32 = 64 * PIXEL_SIZE;
pub const SCREEN_HEIGHT: u32 = 32 * PIXEL_SIZE;

pub struct ScreenBuffer([[bool; 64]; 32]);
impl ScreenBuffer {
    pub fn new() -> Self {
        ScreenBuffer([[false; 64]; 32])
    }
}
impl Deref for ScreenBuffer {
    type Target = [[bool; 64]; 32];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ScreenBuffer {
    fn deref_mut(&mut self) -> &mut [[bool; 64]; 32] {
        &mut self.0
    }
}

pub struct C8Display {
    pub canvas: WindowCanvas,
    on_color: Color,
    off_color: Color,
    debug_lines: bool,
}
impl C8Display {
    pub fn new(
        window: Window,
        on_color: Color,
        off_color: Color,
        debug_lines: bool,
    ) -> Result<Self, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(C8Display {
            canvas,
            on_color,
            off_color,
            debug_lines,
        })
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);

        let pixel = Rect::new(
            x * PIXEL_SIZE as i32,
            y * PIXEL_SIZE as i32,
            PIXEL_SIZE,
            PIXEL_SIZE,
        );

        self.canvas.fill_rect(pixel)?;

        if self.debug_lines {
            self.canvas.set_draw_color(Color::GRAY);
            self.canvas.draw_rect(pixel)?;
        }

        Ok(())
    }

    pub fn draw_screen_buffer(&mut self, buff: ScreenBuffer) -> Result<(), String> {
        for x in 0..64 {
            for y in 0..32 {
                let color = match buff.0[y][x] {
                    true => self.on_color,
                    false => self.off_color,
                };
                self.draw_pixel(x as i32, y as i32, color)?;
            }
        }
        Ok(())
    }
}
