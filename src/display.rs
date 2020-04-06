use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::Sdl;

pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 64;
pub const SCALE_FACTOR: u32 = 10;

#[derive(PartialEq, Debug)]
pub enum DisplayMode {
    Normal,
    Extended,
}

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
        let window = video
            .window(
                "Chip-8",
                WIDTH as u32 * SCALE_FACTOR,
                HEIGHT as u32 * SCALE_FACTOR,
            )
            .position_centered()
            .build()
            .expect("Window creation failed");
        let renderer = window
            .renderer()
            .accelerated()
            .build()
            .expect("Initialization of window renderer failed");

        Display {
            renderer,
            pixels: [[false; WIDTH]; HEIGHT],
            draw_flag: true,
        }
    }

    pub fn draw(&mut self, pixels: &[[bool; WIDTH]; HEIGHT], clamp_pos: i32, clamp_size: u32) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if pixels[y][x] {
                    // Foreground
                    self.renderer.set_draw_color(Color::RGB(251, 241, 199));
                } else {
                    // Background
                    self.renderer.set_draw_color(Color::RGB(69, 133, 149));
                }
                // x, y, w, h
                self.renderer
                    .fill_rect(Rect::new(
                        x as i32 * clamp_pos,
                        y as i32 * clamp_pos,
                        clamp_size as u32,
                        clamp_size as u32,
                    ))
                    .unwrap();
            }
        }
        self.renderer.present();
        self.draw_flag = true;
    }
}
