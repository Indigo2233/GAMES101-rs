use std::rc::Rc;
use std::time::Instant;
use libs::light::Light;
use libs::renderer::Renderer;
use libs::scene::Scene;
use libs::triangle::MeshTriangle;
use libs::vector::Vector3f;

mod libs;

fn main() {
    let mut scene = Scene::new(1280, 960);
    let bunny = MeshTriangle::from_obj(&"./models/bunny/bunny.obj");
    scene.add_obj(Rc::new(bunny));
    scene.add_light(Box::new(Light::new(&Vector3f::new(-20.0, 70.0, 20.0), Vector3f::same(1.0))));
    scene.add_light(Box::new(Light::new(&Vector3f::new(20.0, 70.0, 20.0), Vector3f::same(1.0))));
    scene.build_bvh();

    let start = Instant::now();
    Renderer::render(&scene).unwrap();
    println!("Render complete: ");
    println!("Time taken: {:.2} s", start.elapsed().as_secs_f32());
}
