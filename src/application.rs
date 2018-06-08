use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::time::Duration;

const WINDOW_WIDTH  : u32 = 512;
const WINDOW_HEIGHT : u32 = 512;


fn clear_screen(window: &mut Window, event_pump: &sdl2::EventPump) {
    let mut surface = window.surface(event_pump).unwrap();
    surface.fill_rect(Rect::new(0,0,WINDOW_WIDTH,WINDOW_HEIGHT), Color::RGB(0,0,0)).unwrap();
    surface.finish().unwrap();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("Raytracer", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut window_debug = video_subsystem.window("Debug", WINDOW_WIDTH, WINDOW_WIDTH)
        .position_centered()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
}
