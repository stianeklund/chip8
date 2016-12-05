extern crate sdl2;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// SDL Window TODO: Check if struct contents need pub
pub struct Display<'a> {
    pub renderer: sdl2::render::Renderer<'a>,
    pub screen: [u8; 64 * 32],
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
        let renderer = window.renderer().present_vsync()
            .accelerated()
            .build()
            .unwrap();

        Display {
            renderer: renderer,
            screen: [0; 2048],
            pixels: [[false; 64]; 32],
            draw_flag: true,
        }
    }

    pub fn clear(&mut self) {
        self.draw_flag = true;
        self.pixels = [[false; 64]; 32]
    }

    pub fn draw(& mut self, pixels: &[[bool; 64]; 32]) {
        for y in 0..32 {
            for x in 0..64 {
                if pixels[y][x] {
                    self.renderer.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    self.renderer.set_draw_color(Color::RGB(0, 0, 0));
                }
                // Scaling
                self.renderer.fill_rect(

                    Rect::new(x as i32 * 10, y as i32 * 10, 16 as u32, 16 as u32 )).unwrap();
            }
        }

        self.renderer.present();
        self.draw_flag = true;
    }
}
