use std::os::raw::c_void;
use nalgebra::{Matrix4, Vector3};
use opencv::core::{Mat, MatTraitConst};
use opencv::imgproc::{COLOR_RGB2BGR, cvt_color};

pub(crate) fn get_view_matrix(eye_pos: Vector3<f64>) -> Matrix4<f64> {
    let mut view: Matrix4<f64> = Matrix4::identity();
    view[(0, 3)] = -eye_pos[0];
    view[(1, 3)] = -eye_pos[1];
    view[(2, 3)] = -eye_pos[2];

    view
}

pub(crate) fn get_model_matrix(rotation_angle: f64) -> Matrix4<f64> {
    let mut model: Matrix4<f64> = Matrix4::identity();
    let rad = rotation_angle.to_radians();
    model[(0, 0)] = rad.cos();
    model[(1, 1)] = model[(0, 0)];
    model[(0, 1)] = -rad.sin();
    model[(1, 0)] = -model[(0, 1)];
    model
}

pub(crate) fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
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


pub(crate) fn frame_buffer2cv_mat(frame_buffer: &Vec<Vector3<f64>>) -> opencv::core::Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700, 700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        ).unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image.convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0).expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    img
}