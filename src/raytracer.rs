use camera;
use cgmath::{prelude::*, Vector3};
use light::Light;
use material::Material;
use primitive::plane::Plane;
use primitive::sphere::Sphere;
use scene;
use sdl2::EventPump;
use traceable::*;
use WINDOW_HEIGHT;
use WINDOW_WIDTH;

pub struct Raytracer {
    camera: camera::Camera,
    scene: scene::Scene,
    pub screen: Vec<Vector3<f32>>,
}

const DEPTH: u8 = 4;

impl Raytracer {
    pub fn new() -> Raytracer {
        let mut scene = scene::Scene::new();
        scene.add_primitive(Box::new(Sphere::new(
            Vector3::new(0.0, 1.0, 5.0),
            Material::red_reflective(),
            1.0,
        )));
        scene.add_primitive(Box::new(Sphere::new(
            Vector3::new(-2.0, 1.0, 2.5),
            Material::green_diffuse(),
            1.0,
        )));
        scene.add_primitive(Box::new(Plane::new(
            Vector3::new(0.0, -1.5, 0.0),
            Material::blue_diffuse(),
            Vector3::new(0.0, 1.0, 0.0),
        )));
        scene.add_light(Light {
            position: Vector3::new(0.0, 5.0, 2.5),
            energy: Vector3::new(21.0, 21.0, 21.0),
        });
        Raytracer {
            camera: camera::Camera::new(),
            scene: scene,
            screen: vec![Vector3::zero(); WINDOW_WIDTH * WINDOW_HEIGHT],
        }
    }

    fn clear(&mut self) {
        for i in 0..(WINDOW_WIDTH * WINDOW_HEIGHT) {
            self.screen[i] = Vector3::zero();
        }
    }

    pub fn update(&mut self, delta_time: f32, mut event_pump: &mut EventPump) {
        self.camera.update(delta_time, &mut event_pump);
        let rays = self.camera.shoot_rays();
        let mut traceables: Vec<Traceable> = Vec::new();
        for i in 0..(WINDOW_WIDTH * WINDOW_HEIGHT) {
            let _trace = Traceable {
                ray: &rays[i],
                output: i,
                trace_type: TraceType::PrimaryRay,
                depth: DEPTH,
            };
            traceables.push(_trace);
        }
        self.clear();
        self.render(&mut traceables);
    }

    fn render(&mut self, traceables: &mut Vec<Traceable>) {
        while traceables.len() > 0 {
            let x = traceables.pop();
            match x {
                Some(traceable) => {
                    let oi = &self.scene.intersect(traceable.ray);
                    match oi {
                        Some(intersection) => {
                            let color = self.scene.shadow_rays(&intersection);
                            self.screen[traceable.output] += color;
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }
}
