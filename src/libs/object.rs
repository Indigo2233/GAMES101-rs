use super::bounds3::Bounds3;
use super::intersection::Intersection;
use super::vector::{Vector2f, Vector3f};
use super::ray::Ray;

pub trait Object {
    fn get_intersection(&self, ray: Ray) -> Intersection;
    fn get_surface_properties(&self, p: &Vector3f, q: &Vector3f, index: usize, uv: Vector2f, st: &mut Vector2f) -> Vector3f;
    fn eval_diffuse_color(&self, v: &Vector2f) -> Vector3f;
    fn get_bounds(&self) -> Bounds3;
}
