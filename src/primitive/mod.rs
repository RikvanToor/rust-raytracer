use cgmath::Vector3;
use intersection::Intersection;
use material::Material;
use ray::Ray;

pub mod sphere;
pub mod plane;

pub trait Primitive {
    fn get_position(&self) -> Vector3<f32>;
    // Can't make Primitive a field in Intersection for some reason. To be fixed I hope. TODO
    fn intersect(&self, &Ray, max_distance: f32) -> Option<Intersection>;
    fn get_material(&self) -> &Material;
}
