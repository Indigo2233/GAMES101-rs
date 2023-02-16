use std::collections::HashMap;

use nalgebra::{Matrix4, Vector3};

enum Buffer {
    Color,
    Depth,
    Both,
}

enum Primitive {
    Line,
    Triangle,
}

#[derive(Default)]
struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<int, Vec<Vector3<f64>>>,
    ind_buf: HashMap<int, Vec<Vector3<usize>>>,
    col_buf: HashMap<int, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    width: u64,
    height: u64,
    next_id: usize,
}

struct PosBufId(usize);

struct IndBufId(usize);

struct ColBufId(usize);

impl Rasterizer {
    fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y) * self.width + x) as usize
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (self.height - 1 - point.y) * self.width + point.x;
        let ind = ind as usize;
        self.frame_buf[ind] = *color;
    }

    fn clear(&mut self, buff: Buffer) {
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
    fn set_model(&mut self, model: &Matrix4<f64>) {
        self.model = *model;
    }

    fn set_view(&mut self, view: &Matrix4<f64>) {
        self.view = *view;
    }

    fn set_projection(&mut self, projection: &Matrix4<f64>) {
        self.projection = *projection;
    }
    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }
    fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }
}