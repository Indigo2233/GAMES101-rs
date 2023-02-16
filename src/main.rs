mod triangle;
mod rasterizer;

extern crate opencv;

use std::os::raw::c_void;
use nalgebra::{Matrix4, Vector3};
use opencv::{
    Result,
};
use opencv::core::{Mat, MatTraitConst};
use opencv::highgui::{imshow, wait_key};
use opencv::imgproc::{COLOR_RGB2BGR, cvt_color};
use crate::rasterizer::{Primitive, Rasterizer};

fn get_view_matrix(eye_pos: Vector3<f64>) -> Matrix4<f64> {
    let mut view: Matrix4<f64> = Matrix4::identity();
    view[(0, 3)] = -eye_pos[0];
    view[(1, 3)] = -eye_pos[1];
    view[(2, 3)] = -eye_pos[2];

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
    let mut persp2ortho: Matrix4<f64> = Matrix4::zeros();
    let (n, f) = (z_near, z_far);
    let (a, b) = (n + f, -n * f);
    persp2ortho[(0, 0)] = n;
    persp2ortho[(1, 1)] = n;
    persp2ortho[(3, 2)] = 1.0;
    persp2ortho[(2, 2)] = a;
    persp2ortho[(2, 3)] = b;
    let mut scale: Matrix4<f64> = Matrix4::identity();
    let mut tran: Matrix4<f64> = Matrix4::identity();
    let t = -(eye_fov / 2.0).to_radians().tan() * n.abs();
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

    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![Vector3::new(2.0, 0.0, -2.0),
                   Vector3::new(0.0, 2.0, -2.0),
                   Vector3::new(-2.0, 0.0, -2.0),
                   Vector3::new(3.5, -1.0, -5.0),
                   Vector3::new(2.5, 1.5, -5.0),
                   Vector3::new(-1.0, 0.5, -5.0)];
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

        let mut image = unsafe {
            Mat::new_rows_cols_with_data(
                700, 700,
                opencv::core::CV_64FC3,
                r.frame_buffer().as_ptr() as *mut c_void,
                opencv::core::Mat_AUTO_STEP,
            ).unwrap()
        };
        let mut img = Mat::copy(&image).unwrap();
        image.convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0).expect("panic message");
        cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();


        imshow("image", &image)?;

        k = wait_key(1000).unwrap();
        println!("frame count: {}", frame_count);
        frame_count += 1;
    }

    Ok(())
}