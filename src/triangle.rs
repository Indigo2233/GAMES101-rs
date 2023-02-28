use crate::object::{Attribute, Object};
use crate::vector::{cross, dot, lerp, normalize, Vector2f, Vector3f};

pub struct MeshTriangle {
    vertices: Box<Vec<Vector3f>>,
    vertex_index: Box<Vec<usize>>,
    st_coordinates: Box<Vec<Vector2f>>,
    pub(crate) attr: Attribute,
}

impl MeshTriangle {
    pub fn new(verts: &Vec<Vector3f>, verts_index: &Vec<usize>, st: &Vec<Vector2f>) -> Self {
        MeshTriangle {
            vertices: Box::new(verts.clone()),
            vertex_index: Box::new(verts_index.clone()),
            st_coordinates: Box::new(st.clone()),
            attr: Attribute::new(),
        }
    }
}

fn ray_triangle_intersect(v0: &Vector3f, v1: &Vector3f, v2: &Vector3f,
                          orig: &Vector3f, dir: &Vector3f) -> Option<(f32, f32, f32)> {
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let s = orig - v0;
    let s1 = cross(dir, &e2);
    let s2 = cross(&s, &e1);
    let Vector3f { x: t, y: u, z: v } =
        Vector3f::new(dot(&s2, &e2), dot(&s1, &s), dot(&s2, dir))
            / dot(&s1, &e1);
    if t >= 0.0 && u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
        Some((t, u, v))
    } else {
        None
    }
}

impl Object for MeshTriangle {
    fn intersect(&self, orig: &Vector3f, dir: &Vector3f) -> Option<(f32, usize, Vector2f)> {
        let mut intersect = false;
        let (mut tnear, mut uv, mut index) = (0.0, Vector2f::zeros(), 0);
        for k in 0..self.vertex_index.len() / 3 {
            let v0 = &self.vertices[self.vertex_index[k * 3]];
            let v1 = &self.vertices[self.vertex_index[k * 3 + 1]];
            let v2 = &self.vertices[self.vertex_index[k * 3 + 2]];
            if let Some((t, u, v)) = ray_triangle_intersect(v0, v1, v2, orig, dir) {
                tnear = t;
                uv.x = u;
                uv.y = v;
                index = k;
                intersect |= true;
            }
        }
        if intersect {
            Some((tnear, index, uv))
        } else { None }
    }

    fn get_surface_properties(&self, _p: &Vector3f, _q: &Vector3f,
                              index: usize, uv: Vector2f, st: &mut Vector2f) -> Vector3f {
        let v0 = &self.vertices[self.vertex_index[index * 3]];
        let v1 = &self.vertices[self.vertex_index[index * 3 + 1]];
        let v2 = &self.vertices[self.vertex_index[index * 3 + 2]];
        let e0 = normalize(&(v1 - v0));
        let e1 = normalize(&(v2 - v0));
        let normal = normalize(&cross(&e0, &e1));
        let st0 = &self.st_coordinates[self.vertex_index[index * 3]];
        let st1 = &self.st_coordinates[self.vertex_index[index * 3 + 1]];
        let st2 = &self.st_coordinates[self.vertex_index[index * 3 + 2]];
        *st = st0 * (1.0 - uv.x - uv.y) + st1 * uv.x + st2 * uv.y;
        normal
    }

    fn eval_diffuse_color(&self, st: &Vector2f) -> Vector3f {
        let scale = 5.0;
        let pattern = (((st.x * scale) % 1.0) > 0.5) ^ (((st.y * scale) % 1.0) > 0.5);
        let pattern = pattern as i32 as f32;
        lerp(&Vector3f::new(0.815, 0.235, 0.031), &Vector3f::new(0.937, 0.937, 0.231), pattern)
    }

    fn attribute(&self) -> &Attribute {
        &self.attr
    }
}

