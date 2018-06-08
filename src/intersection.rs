use cgmath::Vector3;
use material::Material;

pub struct Intersection<'a> {
    pub normal: Vector3<f32>,
    pub point: Vector3<f32>,
    pub distance: f32,
    pub material: &'a Material,
}
