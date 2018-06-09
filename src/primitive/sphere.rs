use cgmath::InnerSpace;
use cgmath::Vector3;
use intersection::Intersection;
use material::Material;
use primitive::Primitive;
use ray::Ray;
use std::f32;

pub struct Sphere {
    position: Vector3<f32>,
    material: Material,
    radius: f32,
}

impl Sphere {
    pub fn new(position: Vector3<f32>, material: Material, radius: f32) -> Sphere {
        Sphere {
            position: position,
            material: material,
            radius: radius,
        }
    }
}

impl Primitive for Sphere {
    fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    fn get_material(&self) -> &Material {
        &self.material
    }
    fn intersect(&self, ray: &Ray, max_distance: f32) -> Option<Intersection> {
        let c: Vector3<f32> = self.position - ray.origin;
        let tca = c.dot(ray.direction);
        if tca < 0.0 {
            None
        } else {
            let d2 = c.dot(c) - tca * tca;
            let rsquare = self.radius * self.radius;
            if d2 > rsquare {
                None
            } else {
                let thc = (rsquare - d2).sqrt();
                let t0 = tca - thc;
                let t1 = tca + thc;
                if t0 > 0.0 {
                    if t0 > max_distance {
                        None
                    } else {
                        let p = ray.origin + (t0 - f32::EPSILON) * ray.direction;
                        let n = (p - self.position).normalize();
                        Some(Intersection {
                            normal: n,
                            point: p,
                            distance: t0,
                            material: &self.get_material(),
                        })
                    }
                } else {
                    if t1 > max_distance || t1 < 0.0 {
                        None
                    } else {
                        let p = ray.origin + (t1 - f32::EPSILON) * ray.direction;
                        let n = (p - self.position).normalize();
                        Some(Intersection {
                            normal: n,
                            point: p,
                            distance: t0,
                            material: &self.get_material(),
                        })
                    }
                }
            }
        }
    }
}
