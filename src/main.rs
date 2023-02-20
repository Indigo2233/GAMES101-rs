mod triangle;
mod rasterizer;
mod utils;
mod texture;
mod shader;

extern crate opencv;

use std::env;
use nalgebra::Vector3;
use opencv::{
    Result,
};
use opencv::core::Vector;
use crate::rasterizer::{Buffer, Rasterizer};
use utils::*;
use crate::shader::FragmentShaderPayload;
use crate::texture::Texture;

fn hw3() -> Result<()> {
    let triangles = unsafe { load_triangles() };

    let angle = 140.0;

    let mut r = Rasterizer::new(700, 700);
    let obj_path = "./models/spot/".to_owned();
    let mut filename = "output.png".to_owned();
    let mut texture_path = "hmap.jpg".to_owned();
    let mut tex = Texture::new(&(obj_path.clone() + &texture_path));
    let mut active_shader: fn(&FragmentShaderPayload) -> Vector3<f64> = phong_fragment_shader;
    let ags: Vec<String> = env::args().collect();
    if ags.len() >= 2 {
        filename = ags[1].clone();
        match ags.get(2) {
            None => {}
            Some(method) => {
                if method == "normal" {
                    println!("Rasterizing using the normal shader");
                    active_shader = normal_fragment_shader;
                } else if method == "texture" {
                    println!("Rasterizing using the normal shader");
                    active_shader = texture_fragment_shader;
                    texture_path = "spot_texture.png".to_owned();
                    tex = Texture::new(&(obj_path + &texture_path));
                } else if method == "phong" {
                    println!("Rasterizing using the phong shader");
                    active_shader = phong_fragment_shader;
                } else if method == "bump" {
                    println!("Rasterizing using the bump shader");
                    active_shader = bump_fragment_shader;
                } else if method == "displacement" {
                    println!("Rasterizing using the displacement shader");
                    active_shader = displacement_fragment_shader;
                }
            }
        }
    }
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
    hw3()
}