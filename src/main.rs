mod triangle;
mod rasterizer;

extern crate opencv;

use std::f64::consts::PI;
use nalgebra::{matrix, Matrix4, Vector3};
use opencv::{
    highgui,
    imgcodecs,
    Result,
};

fn get_view_matrix(eye_pos: Vector3<f64>) -> Matrix4<f64> {
    let mut view: Matrix4<f64> = Matrix4::identity();
    view[(0, 3)] = -eye_pos[0];
    view[(1, 3)] = -eye_pos[1];
    view[(2, 3)] = -eye_pos[2];

    println!("{:?}", view);
    view
}

fn get_model_matrix(rotation_angle: f64) -> Matrix4<f64> {
    let mut model: Matrix4<f64> = Matrix4::identity();
    let rad = rotation_angle.to_radians();
    model[(0, 0)] = rad.cos();
    model[(1, 1)] = model[(0, 0)];
    model[(0, 1)] = -rad.sin();
    model[(1, 0)] = -model[(0, 1)];
    model
}

fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
    let projection: Matrix4<f64> = Matrix4::identity();
    let mut persp2ortho: Matrix4<f64> = Matrix4::zeros();
    let (n, f) = (z_near, z_far);
    let (a, b) = (n + f, -n * f);
    persp2ortho[(0, 0)] = n;
    persp2ortho[(1, 1)] = n;
    persp2ortho[(3, 2)] = 1.0;
    persp2ortho[(2, 2)] = a;
    persp2ortho[(2, 3)] = b;
    let mut scale: Matrix4<f64> = Matrix4::zeros();
    let mut tran: Matrix4<f64> = Matrix4::zeros();
    let t = -eye_fov.to_radians().tan() * n.abs();
    let r = aspect_ratio * t;
    let (l, b) = (-r, -t);
    scale[(0, 0)] = 2.0 / (r - l);
    scale[(1, 1)] = 2.0 / (t - b);
    scale[(2, 2)] = 2.0 / (n - f);
    tran[(0, 3)] = -(r + l) / 2.0;
    tran[(1, 3)] = -(t + b) / 2.0;
    tran[(2, 3)] = -(n + f) / 2.0;
    scale * tran * persp2ortho
}

fn main() -> Result<()> {
    // let img_path = "/home/suyao/Pictures/orion_kopa.jpg";
    //
    // let img = imgcodecs::imread(img_path, imgcodecs::IMREAD_COLOR)?;
    // highgui::imshow("Hello opencv!", &img)?;
    // highgui::wait_key(10000)?;
    // Ok(())
    // let m1 = matrix![1, 0, 0;    0, 1, 0; 0, 0, 1];
    // let m2 = matrix![1, 1, 1;
    //     0, 0, 0;
    //     1, 1, 0];
    // println!("{:?}", m1 * m2);


    Ok(())
}