use cgmath::Vector3;

pub struct Material {
    pub color: Vector3<f32>,
    pub reflection: Vector3<f32>,
}

impl Material {
    pub fn new(color: Vector3<f32>, reflection: Vector3<f32>) -> Material {
        Material {
            color: color,
            reflection: reflection,
        }
    }
    pub fn new_color(color: Vector3<f32>) -> Material {
        Material::new(color, Vector3::new(1.0, 1.0, 1.0))
    }
    pub fn red_diffuse() -> Material {
        Material::new_color(Vector3::new(1.0, 0.0, 0.0))
    }
    pub fn red_reflective() -> Material {
        Material::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.5, 0.8, 0.8))
    }
    pub fn green_diffuse() -> Material {
        Material::new_color(Vector3::new(0.0, 1.0, 0.0))
    }
    pub fn blue_diffuse() -> Material {
        Material::new_color(Vector3::new(0.0, 0.0, 1.0))
    }
}
