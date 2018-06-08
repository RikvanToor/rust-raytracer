extern crate cgmath;
extern crate sdl2;

mod camera;
mod intersection;
mod light;
mod material;
mod primitive;
mod ray;
mod raytracer;
mod scene;

use cgmath::Vector3;
use raytracer::Raytracer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use std::time::{SystemTime, Duration};

pub const WINDOW_WIDTH: usize = 512;
pub const WINDOW_HEIGHT: usize = 512;

pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val > max {
        max
    } else if val < min {
        min
    } else {
        val
    }
}

pub fn vec_to_color(vec: Vector3<f32>) -> Color {
    let r = (clamp(vec.x, 0.0, 1.0) * 255.0) as u8;
    let g = (clamp(vec.y, 0.0, 1.0) * 255.0) as u8;
    let b = (clamp(vec.z, 0.0, 1.0) * 255.0) as u8;
    Color::RGB(r, g, b)
}

pub fn test() {
    let mut raytracer = Raytracer::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 resource-manager demo", 512, 512)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().software().build().unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 512, 512)
        .unwrap();
    let mut time = SystemTime::now();

    'mainloop: loop {
        let cur_time = SystemTime::now();
        let delta_time = cur_time.duration_since(time).unwrap_or(Duration::new(0,0));
        let delta_time_f32 = delta_time.as_secs() as f32 + delta_time.subsec_nanos() as f32 * 1e-9;
        time = cur_time;
        raytracer.update(delta_time_f32, &mut sdl_context.event_pump().unwrap());
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                for i in 0..512 {
                    for j in 0..512 {
                        let c = vec_to_color(raytracer.screen[i * 512 + j]);

                        texture_canvas.set_draw_color(c);
                        texture_canvas
                            .fill_rect(Rect::new(j as i32, 511 - i as i32, 1, 1))
                            .unwrap();
                    }
                }
            })
            .unwrap();
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        let dst = Some(Rect::new(0, 0, 512, 512));
        canvas.clear();
        canvas
            .copy_ex(
                &texture,
                None,
                dst,
                0.0,
                Some(Point::new(512, 512)),
                false,
                false,
            )
            .unwrap();
        canvas.present();
    }
}

fn main() {
    test();
}
