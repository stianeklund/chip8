extern crate sdl2;

// use sdl2::render::Renderer;
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

// SDL Window TODO: Check if struct contents need pub
pub struct Display<'a> {
    pub renderer: sdl2::render::Renderer<'a>,
    pub texture: sdl2::render::Texture,
    pub buffer: [u8; 64 * 32],
    pub draw_flag: bool,
}

impl<'a> Display<'a> {
    pub fn new(sdl_context: &Sdl) -> Display<'a> {

        // Initialize SDL2
        // let sdl_context = sdl2::init().expect("init failed in display.rs");
        let video = sdl_context.video().unwrap();
        let mut timer = sdl_context.timer().unwrap();
        let mut events = sdl_context.event_pump().unwrap();

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
            texture: texture,
            buffer: [0; 64 * 32],
            draw_flag: true,
        }
    }

    pub fn draw(&mut self) {
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, Some(Rect::new(0,0, 64, 32)));
        self.renderer.present();
    }
    // Set all pixels in array to 0
    pub fn clear(&mut self) {
        self.buffer = [0; 64 * 32];
        self.draw_flag = true;
    }
}
