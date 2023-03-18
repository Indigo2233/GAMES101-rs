use super::vector::Vector3f;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
    pub direction_inv: Vector3f,
    t: f64,
    t_min: f64,
    t_max: f64,
}

impl Ray {
    pub fn new(origin: Vector3f, direction: Vector3f, t: f64) -> Self {
        let direction_inv = Vector3f::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        let t_min = 0.0;
        let t_max = f64::MAX;
        Self { origin, direction, direction_inv, t, t_min, t_max }
    }

    pub fn at(&self, t: f64) -> Vector3f {
        &self.origin + t as f32 * &self.direction
    }

    pub fn change_dir(&mut self, dir: Vector3f) {
        self.direction_inv.x = 1.0 / dir.x;
        self.direction_inv.y = 1.0 / dir.y;
        self.direction_inv.z = 1.0 / dir.z;
        self.direction = dir;
    }
}