extern crate sdl2;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;
const NPIXELS: usize = (WIDTH * HEIGHT) as usize;

pub struct Display<'a> {
    pub renderer: sdl2::render::Renderer<'a>,
    pub pixels: [[bool; 64]; 32],
    pub draw_flag: bool,
}

impl<'a> Display<'a> {
    pub fn new(sdl_context: &Sdl) -> Display<'a> {

        // Initialize SDL2
        let video = sdl_context.video().unwrap();
        // let mut timer = sdl_context.timer().unwrap();

        // Create window
        let window = video.window("Chip-8", 64*10, 32*10)
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

    pub fn clear(&mut self, pixels: &[[bool; 64]; 32]) {
        for i in 0..NPIXELS {
        self.pixels = [[false; 64]; 32];
        self.draw_flag = true;
        }
    }

    pub fn draw(&mut self, pixels: &[[bool; 64]; 32]) {
        for y in 0..32 {
            for x in 0..64 {
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
        // self.draw_flag = true;
    }
}
