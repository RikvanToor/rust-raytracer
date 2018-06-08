use cgmath::{prelude::*, Vector3};
use intersection::Intersection;
use light::Light;
use primitive::Primitive;
use ray::Ray;
use std::f32;
use std::vec::Vec;

pub struct Scene {
    primitives: Vec<Box<Primitive>>,
    pub lights: Vec<Box<Light>>,
}

impl Scene {
    pub fn new() -> Self {
        let prims: Vec<Box<Primitive>> = Vec::new();
        let lights: Vec<Box<Light>> = Vec::new();
        Self {
            primitives: prims,
            lights: lights,
        }
    }
    pub fn add_primitive(&mut self, prim: Box<Primitive>) {
        self.primitives.push(prim);
    }
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(Box::new(light));
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        for p in &self.primitives {
            let dist = match &result {
                None => f32::MAX,
                Some(x) => x.distance,
            };
            let j = p.intersect(&ray, dist);
            match j {
                None => {}
                Some(x) => {
                    if x.distance < dist {
                        result = Some(x);
                    } else {
                    }
                }
            }
        }
        result
    }

    pub fn shadow_rays(&self, intersection: &Intersection) -> Vector3<f32> {
        let mut color = Vector3::zero();
        let origin = intersection.point + f32::EPSILON * intersection.normal;
        for bl in &self.lights {
            let l = &*bl;
            let direction = l.position - origin;
            let length = direction.magnitude();
            let direction = direction / length;
            let shadow_ray = Ray {
                origin: origin,
                direction: direction,
            };
            match (self).intersect(&shadow_ray) {
                Some(shadow_intersection) => {
                    if shadow_intersection.distance > length - f32::EPSILON {
                        color += get_color(&intersection, &direction, length, &l.energy);
                    } else {
                    }
                }
                None => {
                    color += get_color(&intersection, &direction, length, &l.energy);
                }
            }
        }

        fn get_color(
            intersection: &Intersection,
            shadow_ray_direction: &Vector3<f32>,
            light_distance: f32,
            light_energy: &Vector3<f32>,
        ) -> Vector3<f32> {
            let ndotl = intersection.normal.dot(*shadow_ray_direction);
            let distance2 = light_distance * light_distance;
            let brightness = ndotl / distance2 * light_energy;
            Vector3::new(
                intersection.material.color.x * brightness.x,
                intersection.material.color.y * brightness.y,
                intersection.material.color.z * brightness.z,
            )
        }

        color
    }
}
