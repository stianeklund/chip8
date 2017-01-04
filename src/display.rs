extern crate sdl2;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display<'a> {
    pub renderer: sdl2::render::Renderer<'a>,
    pub pixels: [[bool; 64]; 32],
    pub draw_flag: bool,
}

impl<'a> Display<'a> {
    pub fn new(sdl_context: &Sdl) -> Display<'a> {

        // Initialize SDL2
        let video = sdl_context.video().unwrap();

        // Create window
        let window = video.window("Chip-8", WIDTH as u32 * 10, HEIGHT as u32 * 10)
            .position_centered()
            .build()
            .expect("Window creation failed");
        let renderer = window.renderer()
            .accelerated()
            .build()
            .unwrap();

        Display {
            renderer: renderer,
            pixels: [[false; 64]; 32],
            draw_flag: true,
        }
    }

    pub fn draw(&mut self, pixels: &[[bool; 64]; 32]) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if pixels[y][x] {
                    self.renderer.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    self.renderer.set_draw_color(Color::RGB(0, 0, 0));
                }
                // Scaling
                self.renderer.fill_rect(
                    Rect::new(x as i32 * 10, y as i32 * 10, 15 as u32, 15 as u32 )).unwrap();
            }
        }
        self.renderer.present();
        self.draw_flag = true;
    }
}
