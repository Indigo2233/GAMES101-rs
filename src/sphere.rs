use crate::global::solve_quadratic;
use crate::object::{Attribute, Object};
use crate::vector::{dot, normalize, Vector2f, Vector3f};

pub struct Sphere {
    pub(crate) attr: Attribute,
    center: Vector3f,
    radius: f32,
    radius2: f32,
}

impl Sphere {
    pub fn new(center: &Vector3f, radius: f32) -> Self {
        Self { attr: Attribute::new(), center: center.clone(), radius, radius2: radius * radius }
    }
}

impl Object for Sphere {
    fn intersect(&self, orig: &Vector3f, dir: &Vector3f) -> Option<(f32, usize, Vector2f)> {
        let l = orig - &self.center;
        let a = dot(dir, dir);
        let b = 2.0 * dot(dir, &l);
        let c = dot(&l, &l) - self.radius2;
        if let Some((mut t0, t1)) = solve_quadratic(a, b, c) {
            if t0 < 0.0 { t0 = t1; }
            if t0 < 0.0 { None } else { Some((t0, 0, Vector2f::zeros())) }
        } else { None }
    }

    fn get_surface_properties(&self, p: &Vector3f, _q: &Vector3f, _index: usize, _uv: Vector2f, _st: &mut Vector2f) -> Vector3f {
        normalize(&(p - &self.center))
    }

    fn eval_diffuse_color(&self, _v: &Vector2f) -> Vector3f {
        self.attr.diffuse_color.clone()
    }

    fn attribute(&self) -> &Attribute {
        &self.attr
    }
}