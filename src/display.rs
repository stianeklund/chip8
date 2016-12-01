extern crate sdl2;

use sdl2::render::Renderer;
use sdl2::Sdl;
use sdl2::pixels::{Color,PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use cpu::Cpu;

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
        // let sdl_context = sdl2::init().expect("init failed in display.rs");
        let video = sdl_context.video().unwrap();
        let mut timer = sdl_context.timer().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        // Create window
        let window = video.window("Chip-8", 64*10, 32*10)
            .position_centered()
            .build()
            .expect("Window creation failed");
        let mut renderer = window.renderer()
            .accelerated()
            .build()
            .unwrap();
        let texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, 64, 32).unwrap();

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
    pub fn render(& mut self, sprites: &[[bool; 64]; 32]) {
        self.renderer.clear();
        for y in 0..32 {
            for x in 0..64 {
                if sprites[y][x] {
                    self.renderer.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    self.renderer.set_draw_color(Color::RGB(255, 255, 255));
                }
                self.renderer.draw_point(Point::new(x as i32, y as i32));
            }
        }
        self.renderer.present();
    }
    pub fn draw(&mut self, renderer: &mut sdl2::render::Renderer) {
        for x in 0..64{
            for y in 0..32 {
                if self.pixels[x as usize][y as usize] {
                    renderer.set_draw_color(Color::RGB(109, 170, 44));
                } else {
                    renderer.set_draw_color(Color::RGB(2, 95, 95));
                }
                renderer.fill_rect(Rect::new(
                    x as i32, y as i32, 64 as u32, 32 as u32)).unwrap();
                                   renderer.present();
                self.draw_flag = false;
            }
        }
    }
}
