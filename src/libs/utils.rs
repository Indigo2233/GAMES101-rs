use std::sync::Arc;
use super::material::Material;
use super::bounds3::Bounds3;
use super::triangle::Triangle;
use super::vector::Vector3f;

pub fn load_triangles(obj_file: &str, m: Arc<Material>) -> (Bounds3, Vec<Triangle>, f32) {
    let (models, _) = tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).unwrap();
    let mesh = &models[0].mesh;
    let n = mesh.indices.len() / 3;
    let mut triangles = Vec::with_capacity(n);
    let mut bounding_box = Bounds3::empty();
    let mut area = 0.0;

    for vtx in 0..n {
        let idx: Vec<_> = mesh.indices[vtx * 3..vtx * 3 + 3].iter().map(|i| *i as usize).collect();
        let mut face_vertices = [Vector3f::zeros(), Vector3f::zeros(), Vector3f::zeros()];
        for j in 0..3 {
            let vert = &mesh.positions[3 * idx[j]..3 * idx[j] + 3];
            face_vertices[j] = Vector3f::new(vert[0], vert[1], vert[2]);
            bounding_box.p_min = Vector3f::min(&bounding_box.p_min, &face_vertices[j]);
            bounding_box.p_max = Vector3f::max(&bounding_box.p_max, &face_vertices[j]);
        }
        let [v0, v1, v2] = face_vertices;
        let t = Triangle::new(v0, v1, v2, Some(m.clone()));
        area += t.area;
        triangles.push(t);
    }

    (bounding_box, triangles, area)
}