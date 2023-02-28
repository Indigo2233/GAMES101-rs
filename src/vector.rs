use std::ops::{Div, Mul, Sub, Add, Neg, AddAssign};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn zeros() -> Self {
        Vector3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn same(xx: f32) -> Self {
        Vector3f {
            x: xx,
            y: xx,
            z: xx,
        }
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3f { x, y, z }
    }
}

impl Mul<&Vector3f> for Vector3f {
    type Output = Vector3f;

    fn mul(self, v: &Vector3f) -> Self::Output {
        Vector3f::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn mul(self, v: Vector3f) -> Self::Output {
        Vector3f::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<&Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn mul(self, v: &Vector3f) -> Self::Output {
        Vector3f::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn mul(self, v: Vector3f) -> Self::Output {
        Vector3f::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Sub<&Vector3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, v: &Vector3f) -> Self::Output {
        Vector3f::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Sub<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn sub(self, v: Vector3f) -> Self::Output {
        Vector3f::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Sub<&Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn sub(self, v: &Vector3f) -> Self::Output {
        Vector3f::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Sub<Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn sub(self, v: Vector3f) -> Self::Output {
        Vector3f::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Add<&Vector3f> for Vector3f {
    type Output = Vector3f;

    fn add(self, v: &Vector3f) -> Self::Output {
        Vector3f::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Add<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn add(self, v: Vector3f) -> Self::Output {
        Vector3f::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Add<&Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn add(self, v: &Vector3f) -> Self::Output {
        Vector3f::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Add<Vector3f> for &Vector3f {
    type Output = Vector3f;

    fn add(self, v: Vector3f) -> Self::Output {
        Vector3f::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl AddAssign<&Vector3f> for Vector3f {
    fn add_assign(&mut self, v: &Vector3f) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl AddAssign<Vector3f> for Vector3f {
    fn add_assign(&mut self, v: Vector3f) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl Neg for Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Self::Output {
        Vector3f::new(-self.x, -self.y, -self.z)
    }
}

impl Neg for &Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Self::Output {
        Vector3f::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vector3f {
    type Output = Vector3f;

    fn mul(self, v: f32) -> Self::Output {
        Vector3f::new(self.x * v, self.y * v, self.z * v)
    }
}

impl Mul<f32> for &Vector3f {
    type Output = Vector3f;

    fn mul(self, v: f32) -> Self::Output {
        Vector3f::new(self.x * v, self.y * v, self.z * v)
    }
}

impl Div<f32> for Vector3f {
    type Output = Vector3f;

    fn div(self, v: f32) -> Self::Output {
        Vector3f::new(self.x / v, self.y / v, self.z / v)
    }
}

impl Mul<Vector3f> for f32 {
    type Output = Vector3f;

    fn mul(self, v: Vector3f) -> Self::Output {
        Vector3f::new(self * v.x, self * v.y, self * v.z)
    }
}

impl Mul<&Vector3f> for f32 {
    type Output = Vector3f;

    fn mul(self, v: &Vector3f) -> Self::Output {
        Vector3f::new(self * v.x, self * v.y, self * v.z)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector2f {
    pub fn zeros() -> Self {
        Vector2f {
            x: 0.0,
            y: 0.0,
        }
    }
    pub fn same(xx: f32) -> Self {
        Vector2f {
            x: xx,
            y: xx,
        }
    }
    pub fn new(x: f32, y: f32) -> Self {
        Vector2f { x, y }
    }
}

impl Mul<f32> for Vector2f {
    type Output = Vector2f;

    fn mul(self, v: f32) -> Self::Output {
        Vector2f::new(self.x * v, self.y * v)
    }
}

impl Mul<f32> for &Vector2f {
    type Output = Vector2f;

    fn mul(self, v: f32) -> Self::Output {
        Vector2f::new(self.x * v, self.y * v)
    }
}

impl Add<&Vector2f> for Vector2f {
    type Output = Vector2f;

    fn add(self, v: &Vector2f) -> Self::Output {
        Vector2f::new(self.x + v.x, self.y + v.y)
    }
}

impl Add<Vector2f> for Vector2f {
    type Output = Vector2f;

    fn add(self, v: Vector2f) -> Self::Output {
        Vector2f::new(self.x + v.x, self.y + v.y)
    }
}

pub fn lerp(a: &Vector3f, b: &Vector3f, t: f32) -> Vector3f {
    a * (1.0 - t) + b * t
}

pub fn normalize(v: &Vector3f) -> Vector3f {
    let mag2 = v.x * v.x + v.y * v.y + v.z * v.z;
    if mag2 > 0.0 {
        let inv_mag = 1.0 / mag2.sqrt();
        Vector3f::new(v.x * inv_mag, v.y * inv_mag, v.z * inv_mag)
    } else { v.clone() }
}

pub fn dot(a: &Vector3f, b: &Vector3f) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: &Vector3f, b: &Vector3f) -> Vector3f {
    Vector3f::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
}
