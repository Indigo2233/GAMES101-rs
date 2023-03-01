use std::env::current_dir;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;
use std::slice;
use crate::bounds3::Bounds3;
use crate::global::MaterialType;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vector::Vector3f;


#[link(name = "objloader")]
extern "C" {
    fn create_new_loader() -> *const c_void;
    fn delete_loader(loader: *const c_void);
    fn load_file(loader: *const c_void, file: *const c_char) -> i32;
    fn loaded_meshes(loader: *const c_void, nmesh: *mut i32) -> *const c_void;
    fn mesh_at(meshes: *const c_void, idx: usize) -> *const c_void;
    fn vertex_size_mesh(mesh: *const c_void) -> usize;
    fn mesh_position_at(mesh: *const c_void, idx: usize) -> *const f32;
    fn mesh_normal_at(mesh: *const c_void, idx: usize) -> *const f32;
    fn mesh_texture_at(mesh: *const c_void, idx: usize) -> *const f32;
}

pub unsafe fn load_triangles(filename: &str) -> (Bounds3, Vec<Triangle>) {
    let loader = create_new_loader();
    let mut triangles = vec![];

    let file: *const c_char = CString::new(filename).unwrap().into_raw();
    println!("{:?}", current_dir().unwrap());
    load_file(loader, file);
    let mut nmesh: i32 = 0;
    let meshes = loaded_meshes(loader, &mut nmesh as *mut i32);
    assert_eq!(nmesh, 1);
    let mesh = mesh_at(meshes, 0);
    let sz = vertex_size_mesh(mesh);
    let mut min_vert = Vector3f::same(f32::MAX);
    let mut max_vert = Vector3f::same(f32::MIN);
    let mut j = 0;
    let mut material = Material::new();
    material.material_type = MaterialType::DiffuseAndGlossy;
    material.m_color = Vector3f::new(0.5, 0.5, 0.5);
    material.m_emission = Vector3f::zeros();
    material.kd = 0.6;
    material.ks = 0.0;
    material.specular_exponent = 0.0;
    let mat = Rc::new(material);
    while j < sz {
        let mut face_vertices = [Vector3f::zeros(), Vector3f::zeros(), Vector3f::zeros()];
        for k in 0..3 {
            let vert: Vec<f64> = slice::from_raw_parts(mesh_position_at(mesh, k + j), 3)
                .into_iter().map(|elem| *elem as f64).collect();
            face_vertices[k] = Vector3f::new(vert[0] as f32, vert[1] as f32, vert[2] as f32);
            min_vert = Vector3f::min(&min_vert, &face_vertices[k]);
            max_vert = Vector3f::min(&max_vert, &face_vertices[k]);
        }
        j += 3;
        let [v0, v1, v2] = face_vertices;
        triangles.push(Triangle::new(v0, v1, v2, Some(mat.clone())));
    }
    let bounding_box = Bounds3::new(min_vert, max_vert);

    delete_loader(loader);
    (bounding_box, triangles)
}