mod triangle;
mod rasterizer;
mod utils;
mod texture;
mod shader;

extern crate opencv;

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

use std::ffi::{c_char, c_void, CString};
use std::slice;
use nalgebra::{Vector3, Vector4};
use opencv::{
    Result,
};
use opencv::core::Vector;
use crate::rasterizer::{Buffer, Rasterizer};
use utils::*;
use crate::texture::Texture;
use crate::triangle::Triangle;

unsafe fn load_triangles() -> Vec<Triangle> {
    let mut triangles = vec![];
    let loader = create_new_loader();

    let file: *const c_char = CString::new("./models/spot/spot_triangulated_good.obj").unwrap().into_raw();
    load_file(loader, file);

    let mut nmesh: i32 = 0;
    let meshes = loaded_meshes(loader, &mut nmesh as *mut i32);
    for i in 0..nmesh as usize {
        let mesh = mesh_at(meshes, i);
        let sz = vertex_size_mesh(mesh);
        let mut j = 0;
        while j < sz {
            let mut t = Triangle::default();
            for k in 0..3 {
                let res: Vec<f64> = slice::from_raw_parts(mesh_position_at(mesh, k + j), 3)
                    .into_iter().map(|elem| *elem as f64).collect();
                t.set_vertex(k, Vector4::new(res[0], res[1], res[2], 1.0));

                let res: Vec<f64> = slice::from_raw_parts(mesh_normal_at(mesh, k + j), 3)
                    .into_iter().map(|elem| *elem as f64).collect();
                t.set_normal(k, Vector3::new(res[0], res[1], res[2]));
                let res: Vec<f64> = slice::from_raw_parts(mesh_texture_at(mesh, k + j), 2)
                    .into_iter().map(|elem| *elem as f64).collect();
                t.set_tex_coord(k, res[0], res[1]);
            }
            j += 3;

            triangles.push(t);
        }
    }

    delete_loader(loader);
    triangles
}

fn hw3() -> Result<()> {
    let triangles = unsafe { load_triangles() };

    let angle = 140.0;

    let mut r = Rasterizer::new(700, 700);
    let obj_path = "./models/spot/".to_owned();
    let filename = "output.png".to_owned();
    let texture_path = "hmap.jpg".to_owned();
    let tex = Texture::new(&(obj_path + &texture_path));
    let mut active_shader = phong_fragment_shader;
    r.set_texture(tex);

    let eye_pos = Vector3::new(0.0, 0.0, 10.0);
    r.set_vertex_shader(vertex_shader);
    r.set_fragment_shader(active_shader);


    r.clear(Buffer::Both);
    r.set_model(get_model_matrix(angle));
    r.set_view(get_view_matrix(eye_pos));
    r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));

    r.draw(&triangles);

    let image = frame_buffer2cv_mat(r.frame_buffer());
    let v: Vector<i32> = Default::default();

    opencv::imgcodecs::imwrite(&filename, &image, &v).unwrap();
    Ok(())
}

fn main() -> Result<()> {
    // hw2()
    hw3()
}