use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;


const DISPLAY_SCALE: u32 = 15;
const DISPLAY_WIDTH: u32 = 64 * DISPLAY_SCALE;
const DISPLAY_HEIGHT: u32 = 32 * DISPLAY_SCALE;

pub struct Display {
    canvas: Canvas<Window>
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display { canvas: canvas }
    }

    pub fn draw(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.clear();
        self.canvas.present();
    }
}