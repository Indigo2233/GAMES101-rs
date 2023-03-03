use std::f32::consts::PI;
use crate::libs::global::get_random_float;
use crate::libs::renderer::EPSILON;
use crate::libs::vector::{cross, dot, norm};
use super::global::MaterialType;
use super::vector::Vector3f;

#[derive(Debug)]
pub struct Material {
    pub material_type: MaterialType,
    pub m_color: Vector3f,
    pub m_emission: Vector3f,
    pub ior: f32,
    pub kd: Vector3f,
    pub ks: Vector3f,
    pub diffuse_color: Vector3f,
    pub specular_exponent: f32,
}

impl Material {
    pub fn new(material_type: MaterialType, c: Vector3f, e: Vector3f) -> Self {
        Material {
            material_type,
            m_color: c,
            m_emission: e,
            ior: 1.3,
            kd: Vector3f::zeros(),
            ks: Vector3f::zeros(),
            diffuse_color: Vector3f::same(0.2),
            specular_exponent: 25.0,
        }
    }
    pub fn has_emission(&self) -> bool {
        norm(&self.m_emission) > EPSILON
    }
    pub fn get_emission(&self) -> &Vector3f {
        &self.m_emission
    }
    pub fn sample(&self, _wi: &Vector3f, normal: &Vector3f) -> Vector3f {
        match self.material_type {
            MaterialType::Diffuse => {
                let x1 = get_random_float();
                let x2 = get_random_float();
                let z = (1.0 - 2.0 * x1).abs();
                let r = (1.0 - z * z).sqrt();
                let phi = 2.0 * PI * x2;
                let local_ray = Vector3f::new(r * phi.cos(), r * phi.sin(), z);
                Self::go_world(&local_ray, &normal)
            }
        }
    }
    pub fn eval(&self, _wi: &Vector3f, wo: &Vector3f, normal: &Vector3f) -> Vector3f {
        match self.material_type {
            MaterialType::Diffuse => {
                let cos_alpha = dot(&normal, &wo);
                if cos_alpha > 0.0 {
                    let diffuse = &self.kd / PI;
                    diffuse
                } else { Vector3f::zeros() }
            }
        }
    }
    pub fn pdf(&self, _wi: &Vector3f, wo: &Vector3f, normal: &Vector3f) -> f32 {
        match self.material_type {
            MaterialType::Diffuse => {
                if dot(&wo, &normal) > 0.0 {
                    0.5 / PI
                } else { 0.0 }
            }
        }
    }
    fn go_world(a: &Vector3f, normal: &Vector3f) -> Vector3f {
        let (x, y, z) = (normal.x, normal.y, normal.z);
        let c = if normal.x.abs() > normal.y.abs() {
            let inv_len = 1.0 / (x * x + z * z).sqrt();
            Vector3f::new(z * inv_len, 0.0, -x * inv_len)
        } else {
            let inv_len = 1.0 / (y * y + z * z).sqrt();
            Vector3f::new(0.0, z * inv_len, -y * inv_len)
        };
        let b = cross(&c, &normal);
        a.x * b + a.y * c + a.z * normal
    }
}