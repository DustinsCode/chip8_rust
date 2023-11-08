extern crate sdl2;

mod display;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use display::Display;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    
    let mut display = Display::new(&sdl_context);
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        display.draw(i, 62, 255-i);
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}