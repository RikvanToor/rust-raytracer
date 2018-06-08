use cgmath::InnerSpace;
use cgmath::Vector3;
use intersection::Intersection;
use material::Material;
use primitive::Primitive;
use ray::Ray;

pub struct Plane {
    position: Vector3<f32>,
    material: Material,
    normal: Vector3<f32>,
}

impl Plane {
    pub fn new(position: Vector3<f32>, material: Material, normal: Vector3<f32>) -> Plane {
        Plane {
            position: position,
            material: material,
            normal: normal,
        }
    }
}

impl Primitive for Plane {
    fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    fn get_material(&self) -> &Material {
        &self.material
    }
    fn intersect(&self, ray: &Ray, max_distance: f32) -> Option<Intersection> {
        let c = ray.direction.dot(self.normal);
        if c == 0.0 {
            None
        } else {
            let w = ray.origin - self.position;
            let fac = -self.normal.dot(w) / c;
            if fac > 1.0 && fac < max_distance {
                Some(Intersection {
                    normal: self.normal,
                    point: ray.origin + fac * ray.direction,
                    distance: fac,
                    material: self.get_material(),
                })
            } else {
                None
            }
        }

    }
}
