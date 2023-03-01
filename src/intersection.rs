use std::rc::Rc;
use crate::material::Material;
use crate::object::Object;
use crate::vector::Vector3f;

pub struct Intersection {
    pub happened: bool,
    pub coords: Vector3f,
    pub normal: Vector3f,
    pub distance: f64,
    pub obj: Option<Rc<dyn Object>>,
    pub m: Option<Rc<Material>>,
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            happened: false,
            coords: Vector3f::zeros(),
            normal: Vector3f::zeros(),
            distance: f64::MAX,
            obj: None,
            m: None,
        }
    }
}