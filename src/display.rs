use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;

use egui_sdl2::EguiCanvas;

const PIXEL_SIZE: u32 = 20;
pub const SCREEN_WIDTH: u32 = 64 * PIXEL_SIZE;
pub const SCREEN_HEIGHT: u32 = 32 * PIXEL_SIZE;

pub struct C8Display {
    pub canvas: EguiCanvas,
    pub buff: [bool; 64 * 32],
    pub on_color: Color,
    pub off_color: Color,
    pub debug_lines: bool,
}
impl C8Display {
    pub fn new(
        window: Window,
        on_color: Color,
        off_color: Color,
        debug_lines: bool,
    ) -> Result<Self, String> {
        let canvas = EguiCanvas::new(window);
        Ok(C8Display {
            canvas,
            buff: [false; 64 * 32],
            on_color,
            off_color,
            debug_lines,
        })
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), String> {
        self.canvas.painter.canvas.set_draw_color(color);

        let pixel = Rect::new(
            x * PIXEL_SIZE as i32 + SCREEN_WIDTH as i32 / 2, // TODO: hacky
            y * PIXEL_SIZE as i32 + SCREEN_HEIGHT as i32 / 2,
            PIXEL_SIZE,
            PIXEL_SIZE,
        );

        self.canvas.painter.canvas.fill_rect(pixel)?;

        if self.debug_lines {
            self.canvas.painter.canvas.set_draw_color(Color::GRAY);
            self.canvas.painter.canvas.draw_rect(pixel)?;
        }

        Ok(())
    }

    pub fn draw_screen_buffer(&mut self) -> Result<(), String> {
        for x in 0..64 {
            for y in 0..32 {
                let color = match self.buff[y * 64 + x] {
                    true => self.on_color,
                    false => self.off_color,
                };
                self.draw_pixel(x as i32, y as i32, color)?;
            }
        }
        self.canvas.present();
        Ok(())
    }
}
