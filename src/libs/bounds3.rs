use std::mem::swap;
use super::ray::Ray;
use super::vector::Vector3f;

#[derive(Clone, Debug)]
pub struct Bounds3 {
    pub p_min: Vector3f,
    pub p_max: Vector3f,
}

pub enum Axis { X, Y, Z }

impl Bounds3 {
    pub fn new(p1: Vector3f, p2: Vector3f) -> Self {
        let p_min = Vector3f::new(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z));
        let p_max = Vector3f::new(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z));
        Self { p_min, p_max }
    }
    pub fn empty() -> Self {
        Self { p_min: Vector3f::same(f32::MAX), p_max: Vector3f::same(f32::MIN) }
    }

    pub fn diagonal(&self) -> Vector3f { &self.p_max - &self.p_min }
    pub fn max_extent(&self) -> Axis {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z { Axis::X } else if d.y > d.z { Axis::Y } else { Axis::Z }
    }
    #[allow(dead_code)]
    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.y * d.z + d.z * d.x) as f64
    }
    pub fn centroid(&self) -> Vector3f { 0.5 * &self.p_min + 0.5 * &self.p_max }
    #[allow(dead_code)]
    pub fn intersect(&self, b: &Bounds3) -> Bounds3 {
        Bounds3 {
            p_min: Vector3f::max(&self.p_min, &b.p_min),
            p_max: Vector3f::min(&self.p_max, &b.p_max),
        }
    }
    #[allow(dead_code)]
    pub fn offset(&self, p: &Vector3f) -> Vector3f {
        let mut o = p - &self.p_min;
        if self.p_max.x > self.p_min.x { o.x /= self.p_max.x - self.p_min.x; }
        if self.p_max.y > self.p_min.y { o.y /= self.p_max.y - self.p_min.y; }
        if self.p_max.z > self.p_min.z { o.z /= self.p_max.z - self.p_min.z; }
        o
    }
    #[allow(dead_code)]
    pub fn overlaps(b1: &Bounds3, b2: &Bounds3) -> bool {
        let x = b1.p_max.x >= b2.p_min.x && b1.p_min.x <= b2.p_max.x;
        let y = b1.p_max.y >= b2.p_min.y && b1.p_min.y <= b2.p_max.y;
        let z = b1.p_max.z >= b2.p_min.z && b1.p_min.z <= b2.p_max.z;
        x && y && z
    }
    #[allow(dead_code)]
    pub fn inside(p: &Vector3f, b: &Bounds3) -> bool {
        p.x >= b.p_min.x && p.x <= b.p_max.x &&
            p.y >= b.p_min.y && p.y <= b.p_max.y &&
            p.z >= b.p_min.z && p.z <= b.p_max.z
    }
    pub fn intersect_p(&self, ray: &Ray, inv_dir: &Vector3f, dir_neg: [bool; 3]) -> bool {
        let mut t_min = (&self.p_min - &ray.origin) * inv_dir;
        let mut t_max = (&self.p_max - &ray.origin) * inv_dir;
        if dir_neg[0] { swap(&mut t_min.x, &mut t_max.x); }
        if dir_neg[1] { swap(&mut t_min.y, &mut t_max.y); }
        if dir_neg[2] { swap(&mut t_min.z, &mut t_max.z); }
        let t_enter = t_min.x.max(t_min.y).max(t_min.z);
        let t_exit = t_max.x.min(t_max.y).min(t_max.z);

        t_enter < t_exit && t_exit >= 0.0
    }
    pub fn union_bounds(b1: &Bounds3, b2: &Bounds3) -> Bounds3 {
        Bounds3 {
            p_min: Vector3f::min(&b1.p_min, &b2.p_min),
            p_max: Vector3f::max(&b1.p_max, &b2.p_max),
        }
    }
    pub fn union_point(b: &Bounds3, p: &Vector3f) -> Bounds3 {
        Bounds3 {
            p_min: Vector3f::min(&b.p_min, p),
            p_max: Vector3f::max(&b.p_max, p),
        }
    }
}


impl Default for Bounds3 {
    fn default() -> Self {
        Bounds3 {
            p_min: Vector3f::same(f32::MAX),
            p_max: Vector3f::same(f32::MIN),
        }
    }
}