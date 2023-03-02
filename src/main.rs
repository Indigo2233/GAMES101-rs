use std::rc::Rc;
use crate::light::Light;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::triangle::MeshTriangle;
use crate::vector::Vector3f;

mod triangle;
mod object;
mod global;
mod vector;
mod light;
mod scene;
mod renderer;
mod utils;
mod material;
mod ray;
mod intersection;
mod bounds3;
mod bvh;

fn main() {
    let mut scene = Scene::new(1280, 960);
    let bunny = MeshTriangle::from_obj(&"./models/bunny/bunny.obj");
    scene.add_obj(Rc::new(bunny));
    scene.add_light(Box::new(Light::new(&Vector3f::new(-20.0, 70.0, 20.0), Vector3f::same(1.0))));
    scene.add_light(Box::new(Light::new(&Vector3f::new(20.0, 70.0, 20.0), Vector3f::same(1.0))));
    scene.build_bvh();

    Renderer::render(&scene);
}
