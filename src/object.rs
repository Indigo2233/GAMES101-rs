use crate::global::MaterialType;
use crate::global::MaterialType::DiffuseAndGlossy;
use crate::vector::{Vector2f, Vector3f};

pub trait Object {
    fn intersect(&self, orig: &Vector3f, dir: &Vector3f) -> Option<(f32, usize, Vector2f)>;
    fn get_surface_properties(&self, p: &Vector3f, q: &Vector3f, index: usize, uv: Vector2f, st: &mut Vector2f) -> Vector3f;
    fn eval_diffuse_color(&self, v: &Vector2f) -> Vector3f;
    fn attribute(&self) -> &Attribute;
}

pub struct Attribute {
    pub material_type: MaterialType,
    pub ior: f32,
    pub kd: f32,
    pub ks: f32,
    pub diffuse_color: Vector3f,
    pub specular_exponent: f32,
}

impl Attribute {
    pub fn new() -> Self {
        Attribute {
            material_type: DiffuseAndGlossy,
            ior: 1.3,
            kd: 0.8,
            ks: 0.2,
            diffuse_color: Vector3f::same(0.2),
            specular_exponent: 25.0,
        }
    }
}