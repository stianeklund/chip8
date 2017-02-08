extern crate sdl2;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 64;

#[derive(PartialEq, Debug)]
pub enum DisplayMode { Default, Extended}

pub struct Display<'a> {
    pub renderer: sdl2::render::Renderer<'a>,
    pub pixels: [[bool; WIDTH]; HEIGHT],
    pub draw_flag: bool,
}

impl<'a> Display<'a> {
    pub fn new(sdl_context: &Sdl) -> Display<'a> {

        // Initialize SDL2
        let video = sdl_context.video().expect("SDL2 initialization failed");

        // Create window
        let window = video.window("Chip-8", WIDTH as u32 * 10, HEIGHT as u32 * 10)
            .position_centered()
            .build()
            .expect("Window creation failed");
        let renderer = window.renderer()
            .accelerated()
            .build()
            .expect("Initialization of window renderer failed");

        Display {
            renderer: renderer,
            pixels: [[false; WIDTH]; HEIGHT],
            draw_flag: true,
        }
    }

    pub fn draw(&mut self, pixels: &[[bool; WIDTH]; HEIGHT], scale: i32) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if pixels[y][x] {
                    // Foreground
                    self.renderer.set_draw_color(Color::RGB(251, 241, 199));
                } else {
                    // Background
                    self.renderer.set_draw_color(Color::RGB(69, 133, 149));
                }
                // Allow different scale size depeding on mode
                self.renderer.fill_rect(
                    Rect::new(x as i32 * scale, y as i32 * scale, scale as u32, scale as u32 )).unwrap();
            }
        }
        self.renderer.present();
        self.draw_flag = true;
    }
}
