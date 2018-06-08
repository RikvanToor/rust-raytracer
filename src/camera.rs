use cgmath::prelude::*;
use cgmath::Vector3;
use ray::Ray;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use WINDOW_HEIGHT;
use WINDOW_WIDTH;

pub struct Camera {
    position: Vector3<f32>,
    direction: Vector3<f32>,
    screen00: Vector3<f32>,
    screen01: Vector3<f32>,
    screen10: Vector3<f32>,
    screen_distance: f32,
    up: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
            screen00: Vector3::zero(),
            screen01: Vector3::zero(),
            screen10: Vector3::zero(),
            screen_distance: 1.0,
            up: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    fn shoot_ray(&self, i: usize) -> Ray {
        let x = i % WINDOW_WIDTH;
        let y = i / WINDOW_WIDTH;

        let dh = self.screen10 - self.screen00;
        let dv = self.screen01 - self.screen00;
        let px = x as f32 / WINDOW_WIDTH as f32;
        let py = y as f32 / WINDOW_HEIGHT as f32;

        let p = self.screen00 + px * dh + py * dv;
        let dir = (p - self.position).normalize();

        Ray {
            origin: self.position,
            direction: dir,
        }
    }

    const SIZE: usize = WINDOW_WIDTH * WINDOW_HEIGHT;

    pub fn shoot_rays(&self) -> Vec<Ray> {
        let rays: Vec<Ray> = (0..Camera::SIZE).map(|i| self.shoot_ray(i)).collect();
        rays
    }

    pub fn update(&mut self, delta_time: f32, mut event_pump: &mut EventPump) {
        let aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
        let right = Vector3::new(0.0, 1.0, 0.0).cross(self.direction);
        self.up = self.direction.cross(right);

        self.handle_input(delta_time, &mut event_pump, &right);

        let rasp = right * aspect_ratio;
        let temppos = self.position + self.screen_distance * self.direction;
        self.screen00 = temppos - rasp - self.up;
        self.screen01 = temppos - rasp + self.up;
        self.screen10 = temppos + rasp - self.up;
    }

    fn handle_input(&mut self, delta_time: f32, event_pump: &mut EventPump, right: &Vector3<f32>) {
        let speed = 2.0;
        let mut delta_dir = Vector3::zero();
        let mut delta_pos = Vector3::zero();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    delta_pos -= *right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    delta_pos += *right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    delta_pos += self.direction;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    delta_pos -= self.direction;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    delta_dir += self.up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    delta_dir -= self.up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    delta_dir -= *right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    delta_dir += *right;
                }

                _ => {}
            }
        }

        if delta_pos.magnitude() > 0.0 {
            self.position += delta_pos.normalize() * delta_time * speed;
        } else {
        }
        if delta_dir.magnitude() > 0.0 {
            self.direction += delta_dir.normalize() * delta_time * 0.8;
            self.direction = self.direction.normalize();
        } else {
        }
    }
}
