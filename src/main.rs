mod triangle;
mod rasterizer;
mod utils;

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
use nalgebra::Vector3;
use opencv::{
    Result,
};
use opencv::highgui::{imshow, wait_key};
use crate::rasterizer::{Primitive, Rasterizer};
use utils::*;
use crate::triangle::Triangle;

fn hw2() -> Result<()> {
    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![Vector3::new(2.0, 0.0, -2.0),
                   Vector3::new(0.0, 2.0, -2.0),
                   Vector3::new(-2.0, 0.0, -2.0),
                   Vector3::new(3.5, -1.0, -5.0),
                   Vector3::new(2.5, 1.5, -5.0),
                   Vector3::new(-1.0, 0.5, -1.0)];
    let ind = vec![Vector3::new(0, 1, 2), Vector3::new(3, 4, 5)];
    let cols = vec![Vector3::new(217.0, 238.0, 185.0),
                    Vector3::new(217.0, 238.0, 185.0),
                    Vector3::new(217.0, 238.0, 185.0),
                    Vector3::new(185.0, 217.0, 238.0),
                    Vector3::new(185.0, 217.0, 238.0),
                    Vector3::new(185.0, 217.0, 238.0), ];
    let pos_id = r.load_position(&pos);
    let ind_id = r.load_indices(&ind);
    let col_id = r.load_colors(&cols);
    let mut k = 0;
    let mut frame_count = 0;
    while k != 27 {
        r.clear(rasterizer::Buffer::Both);
        r.set_model(get_model_matrix(0.0));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, col_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer();
        let image = frame_buffer2cv_mat(frame_buffer);

        imshow("image", &image)?;

        k = wait_key(2000).unwrap();
        println!("frame count: {}", frame_count);
        frame_count += 1;
    }
    Ok(())
}

unsafe fn load_triangles() -> Vec<Triangle> {
    let traingles = vec![];
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
            let mut t = Triangle::new();
            for _ in 0..3 {
                let res: Vec<f64> = slice::from_raw_parts(mesh_position_at(mesh, i + j), 3)
                    .into_iter().map(|elem| *elem as f64).collect();
                t.set_vertex(j, Vector3::new(res[0], res[1], res[2]));

                let res: Vec<f64> = slice::from_raw_parts(mesh_normal_at(mesh, i + j), 3)
                    .into_iter().map(|elem| *elem as f64).collect();
                t.set_normal(j, Vector3::new(res[0], res[1], res[2]));
                let res: Vec<f64> = slice::from_raw_parts(mesh_texture_at(mesh, i + j), 2)
                    .into_iter().map(|elem| *elem as f64).collect();
                t.set_tex_coord(j, res[0], res[1]);
            }
            j += 3;
        }
    }


    delete_loader(loader);
    traingles
}

fn hw3() -> Result<()> {
    let triangles = unsafe { load_triangles() };

    Ok(())
}

fn main() -> Result<()> {
    // hw2()
    hw3()
}