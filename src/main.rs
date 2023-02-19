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
    fn mesh_position_at(mesh: *const c_void, idx: usize, axis: u32) -> *const f32;
    fn mesh_normal_at(mesh: *const c_void, idx: usize, axis: u32) -> *const f32;
    fn mesh_texture_at(mesh: *const c_void, idx: usize, axis: u32) -> *const f32;
}

use std::ffi::{c_char, c_void, CString};
use std::slice;
use nalgebra::{Vector3};
use opencv::{
    Result,
};
use opencv::highgui::{imshow, wait_key};
use crate::rasterizer::{Primitive, Rasterizer};
use utils::*;

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

fn hw3() -> Result<()> {
    let loader = unsafe { create_new_loader() };
    unsafe {
        let file: *const c_char = CString::new("./models/spot/spot_triangulated_good.obj").unwrap().into_raw();
        load_file(loader, file);
    }
    let mut nmesh: i32 = 0;

    let meshes = unsafe { loaded_meshes(loader, &mut nmesh as *mut i32) };

    let mesh = unsafe { mesh_at(meshes, 0) };
    let sz = unsafe { vertex_size_mesh(mesh) };
    println!("{sz}");
    unsafe {
        let x = slice::from_raw_parts(mesh_position_at(mesh, 0, 0), 3);
        println!("{:?}", x);
        let x = slice::from_raw_parts(mesh_normal_at(mesh, 0, 0), 3);
        println!("{:?}", x);
        let x = slice::from_raw_parts(mesh_texture_at(mesh, 0, 0), 2);
        println!("{:?}", x);
    }
    unsafe { delete_loader(loader); }
    Ok(())
}

fn main() -> Result<()> {
    // hw2()
    hw3()
}