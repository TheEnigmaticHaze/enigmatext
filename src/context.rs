use sdl2::{
    render::Canvas,
    ttf::Sdl2TtfContext,
    video::Window,
    Sdl,
    EventPump,
    pixels::Color
};

pub struct Settings {
    pub back_color: Color,
    pub text_color: Color,
    pub font_size: u16,
    pub line_spacing: u16
}

pub struct Context {
    pub settings: Settings,
    pub sdl_context: Sdl,
    pub ttf_context: Sdl2TtfContext,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump
}

impl Context {
    pub fn new(settings: Settings) -> Result<Context, String> {
        let sdl_context = sdl2::init().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("enigmatext", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Ok(Context {
            settings,
            sdl_context,
            ttf_context,
            canvas,
            event_pump
        })
    }
}