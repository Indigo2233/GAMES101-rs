use crate::vector::Vector3f;

pub struct Light {
    pub position: Vector3f,
    pub intensity: Vector3f,
}

impl Light {
   pub fn new(p: &Vector3f, i: &Vector3f) -> Self {
        Light {
            position: p.clone(),
            intensity: i.clone(),
        }
    }
}