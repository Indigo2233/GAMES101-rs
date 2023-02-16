#![allow(dead_code)]

use std::cmp::max;
use std::collections::HashMap;

use nalgebra::{ArrayStorage, Const, Matrix, Matrix4, min, Vector3, Vector4};
use opencv::gapi::validate_input_arg;
use crate::rasterizer::Primitive::Triangle;
use crate::triangle;

pub enum Buffer {
    Color,
    Depth,
    Both,
}

pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
struct PosBufId(usize);

#[derive(Clone, Copy)]
struct IndBufId(usize);

#[derive(Clone, Copy)]
struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);
        r
    }

    fn get_index(height: u64, width: u64, x: usize, y: usize) -> usize {
        ((height - 1 - y as u64) * width + x as u64) as usize
    }

    fn set_pixel(height: u64, width: u64, frame_buf: &mut Vec<Vector3<f64>>, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (height as f64 - 1.0 - point.y) * width as f64 + point.x;
        let ind = ind as usize;
        frame_buf[ind] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color =>
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0)),
            Buffer::Depth =>
                self.depth_buf.fill(f64::MIN),
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MIN);
            }
        }
    }
    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }
    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }
    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }
    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, col_buffer: ColBufId, typ: Primitive) {
        let buf = &self.pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.ind_buf[&ind_buffer.0];
        let col = &self.col_buf[&col_buffer.0];
        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;
        let mvp = self.projection * self.view * self.model;
        for i in ind.iter() {
            let mut t = triangle::Triangle::new();
            let mut v = vec![mvp * to_vec4(buf[i[0]], Some(1.0)),
                             mvp * to_vec4(buf[i[1]], Some(1.0)),
                             mvp * to_vec4(buf[i[2]], Some(1.0))];
            for mut vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for mut vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);
            let v = &t.to_vector4();
            let min_x = v[0].x.min(v[1].x).min(v[2].x) as usize;
            let max_x = v[0].x.max(v[1].x).max(v[2].x) as usize;
            let min_y = v[0].y.min(v[1].y).min(v[2].y) as usize;
            let max_y = v[0].y.max(v[1].y).max(v[2].y) as usize;

            // rasterize_triangle
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let (fx, fy) = (x as f64, y as f64);
                    if !inside_triangle(0.5 + fx, 0.5 + fy, &t.v) { continue; }
                    let (a, b, c) = compute_barycentric2d(0.5 + fx, 0.5 + fy, &t.v);
                    let w_reciprocal = 1.0 / (a / v[0].w + b / v[1].w + c / v[2].w);
                    let mut z_interpolated = a * v[0].z / v[0].w + b * v[1].z / v[1].w + c * v[2].z / v[2].w;
                    z_interpolated *= w_reciprocal;
                    if z_interpolated < self.depth_buf[Rasterizer::get_index(self.width, self.height, x, y)] {
                        self.depth_buf[Rasterizer::get_index(self.width, self.height, x, y)] = z_interpolated;
                        Rasterizer::set_pixel(self.width, self.height, &mut self.frame_buf, &Vector3::new(fx, fy, 1.0), &t.get_color());
                    }
                }
            }
        }
    }
    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    let p = Vector3::new(x, y, 0.0);
    let ap = p - v[0];
    let bp = p - v[1];
    let cp = p - v[2];
    let ab = v[1] - v[0];
    let bc = v[2] - v[1];
    let ca = v[0] - v[2];
    let c1 = ab.cross(&ap);
    let c2 = bc.cross(&bp);
    let c3 = ca.cross(&cp);
    (c1.z > 0.0 && c2.z > 0.0 && c3.z > 0.0) || (c1.z < 0.0 && c2.z < 0.0 && c3.z < 0.0)
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y) / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y) / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y) / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y - v[1].x * v[0].y);
    (c1, c2, c3)
}