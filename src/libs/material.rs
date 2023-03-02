use super::global::MaterialType;
use super::vector::Vector3f;

#[derive(Debug)]
pub struct Material {
    pub material_type: MaterialType,
    pub m_color: Vector3f,
    pub m_emission: Vector3f,
    pub ior: f32,
    pub kd: f32,
    pub ks: f32,
    pub diffuse_color: Vector3f,
    pub specular_exponent: f32,
}

impl Material {
    pub fn new() -> Self {
        Material {
            material_type: MaterialType::DiffuseAndGlossy,
            m_color: Vector3f::same(1.0),
            m_emission: Vector3f::same(0.0),
            ior: 1.3,
            kd: 0.8,
            ks: 0.2,
            diffuse_color: Vector3f::same(0.2),
            specular_exponent: 25.0,
        }
    }
}