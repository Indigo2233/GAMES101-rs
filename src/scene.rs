use std::rc::Rc;
use crate::light::Light;
use crate::object::Object;
use crate::vector::Vector3f;

pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub fov: f64,
    pub background_color: Vector3f,
    pub max_depth: i32,
    pub epsilon: f32,
    objects: Vec<Rc<dyn Object>>,
    lights: Vec<Box<Light>>,
}

impl Scene {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            fov: 90.0,
            background_color: Vector3f::new(0.235294, 0.67451, 0.843137),
            max_depth: 5,
            epsilon: 0.00001,
            objects: vec![],
            lights: vec![],
        }
    }
    pub fn add_obj(&mut self, object: Rc<dyn Object>) {
        self.objects.push(object);
    }
    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }

    pub fn objects(&self) -> &Vec<Rc<dyn Object>> {
        &self.objects
    }
    pub fn lights(&self) -> &Vec<Box<Light>> {
        &self.lights
    }
}

