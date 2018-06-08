use cgmath::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}
