use std::rc::Rc;
use crate::global::MaterialType::{DiffuseAndGlossy, ReflectionAndRefraction};
use crate::light::Light;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::triangle::MeshTriangle;
use crate::vector::{Vector2f, Vector3f};

mod triangle;
mod object;
mod global;
mod vector;
mod light;
mod sphere;
mod scene;
mod renderer;

fn main() {
    let mut scene = Scene::new(1280, 960);
    let mut sph1 = Sphere::new(&Vector3f::new(-1.0, 0.0, -12.0), 2.0);
    sph1.attr.material_type = DiffuseAndGlossy;
    sph1.attr.diffuse_color = Vector3f::new(0.6, 0.7, 0.8);
    let mut sph2 = Sphere::new(&Vector3f::new(0.5, -0.5, -8.0), 1.5);
    sph2.attr.material_type = ReflectionAndRefraction;
    sph2.attr.ior = 1.5;
    scene.add_obj(Rc::new(sph1));
    scene.add_obj(Rc::new(sph2));
    let verts = vec![
        Vector3f::new(-5.0, -3.0, -6.0),
        Vector3f::new(5.0, -3.0, -6.0),
        Vector3f::new(5.0, -3.0, -16.0),
        Vector3f::new(-5.0, -3.0, -16.0),
    ];
    let vert_index = vec![0usize, 1, 3, 1, 2, 3];
    let st = vec![
        Vector2f::new(0.0, 0.0),
        Vector2f::new(1.0, 0.0),
        Vector2f::new(1.0, 1.0),
        Vector2f::new(0.0, 1.0),
    ];
    let mut mesh = MeshTriangle::new(&verts, &vert_index, &st);
    mesh.attr.material_type = DiffuseAndGlossy;
    scene.add_obj(Rc::new(mesh));
    scene.add_light(Box::new(Light::new(&Vector3f::new(-20.0, 70.0, 20.0), &Vector3f::same(0.5))));
    scene.add_light(Box::new(Light::new(&Vector3f::new(30.0, 50.0, -12.0), &Vector3f::same(0.5))));
    Renderer::render(&scene);
}
