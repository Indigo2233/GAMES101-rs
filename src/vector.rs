use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    pub fn zeros() -> Self {
        Vector2d {
            x: 0.0,
            y: 0.0,
        }
    }
    pub fn same(xx: f64) -> Self {
        Vector2d {
            x: xx,
            y: xx,
        }
    }
    pub fn new(x: f64, y: f64) -> Self {
        Vector2d { x, y }
    }

    pub fn norm(&self) -> f64 {
        let (x, y) = (self.x, self.y);
        (x * x + y * y).sqrt()
    }

    pub fn unit(&self) -> Vector2d {
        self / self.norm()
    }
}

impl Mul<f64> for Vector2d {
    type Output = Vector2d;

    fn mul(self, v: f64) -> Self::Output {
        Vector2d::new(self.x * v, self.y * v)
    }
}

impl Mul<f64> for &Vector2d {
    type Output = Vector2d;

    fn mul(self, v: f64) -> Self::Output {
        Vector2d::new(self.x * v, self.y * v)
    }
}

impl Mul<Vector2d> for f64 {
    type Output = Vector2d;

    fn mul(self, v: Vector2d) -> Self::Output {
        Vector2d::new(v.x * self, self * v.y)
    }
}

impl Mul<&Vector2d> for f64 {
    type Output = Vector2d;

    fn mul(self, v: &Vector2d) -> Self::Output {
        Vector2d::new(v.x * self, self * v.y)
    }
}

impl Div<f64> for Vector2d {
    type Output = Vector2d;

    fn div(self, v: f64) -> Self::Output {
        Vector2d::new(self.x / v, self.y / v)
    }
}

impl Div<f64> for &Vector2d {
    type Output = Vector2d;

    fn div(self, v: f64) -> Self::Output {
        Vector2d::new(self.x / v, self.y / v)
    }
}

impl Add<&Vector2d> for Vector2d {
    type Output = Vector2d;

    fn add(self, v: &Vector2d) -> Self::Output {
        Vector2d::new(self.x + v.x, self.y + v.y)
    }
}

impl Add<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn add(self, v: Vector2d) -> Self::Output {
        Vector2d::new(self.x + v.x, self.y + v.y)
    }
}

impl Sub<&Vector2d> for Vector2d {
    type Output = Vector2d;

    fn sub(self, v: &Vector2d) -> Self::Output {
        Vector2d::new(self.x - v.x, self.y - v.y)
    }
}

impl Sub<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn sub(self, v: Vector2d) -> Self::Output {
        Vector2d::new(self.x - v.x, self.y - v.y)
    }
}

impl SubAssign for Vector2d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl AddAssign for Vector2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Neg for Vector2d {
    type Output = Vector2d;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl Neg for &Vector2d {
    type Output = Vector2d;

    fn neg(self) -> Self::Output {
        Vector2d { x: -self.x, y: -self.y }
    }
}