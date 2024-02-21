mod text;
mod context;

use context::{Context, Settings};

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use text::{render_text_from_rope, Rope};
use std::time::Duration;

pub fn main() {
    let settings = Settings {
        back_color: Color::RGB(255, 180, 255),
        text_color: Color::RGB(80, 0, 80),
        font_size: 20,
        line_spacing: 4
    };

    let mut context = Context::new(settings).unwrap();

    let mut my_rope = Rope::from_string("01 3456789".to_string());
    my_rope.insert_string_at("hello", 5);
    println!("{:?}", my_rope);

    'running: loop {
        context.canvas.set_draw_color(context.settings.back_color);
        context.canvas.clear();
        
        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        render_text_from_rope(&my_rope, &mut context);

        context.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}