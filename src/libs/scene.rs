use std::sync::Arc;
use super::global::get_random_float;
use super::vector::norm;
use super::bvh::{BVHAccel, SplitMethod};
use super::object::Object;
use super::ray::Ray;
use super::vector::{dot, normalize, Vector3f};
use super::intersection::Intersection;
use super::triangle::MeshTriangle;

pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub fov: f64,
    pub background_color: Vector3f,
    objects: Vec<Arc<MeshTriangle>>,
    bvh: Option<Arc<BVHAccel>>,
    russian_roulette: f32,
}

impl Scene {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            fov: 40.0,
            background_color: Vector3f::new(0.235294, 0.67451, 0.843137),
            objects: vec![],
            bvh: None,
            russian_roulette: 0.8,
        }
    }
    pub fn add_obj(&mut self, object: Arc<MeshTriangle>) {
        self.objects.push(object);
    }

    pub fn build_bvh(&mut self) {
        println!(" - Generating BVH...\n");
        let objs = self.objects.iter().map(|m| -> Arc<dyn Object + Send + Sync> { m.clone() }).collect();
        self.bvh = Some(Arc::new(BVHAccel::new(objs, 1, SplitMethod::Naive)));
    }

    pub fn intersect(&self, ray: &Ray) -> Intersection {
        if let Some(bvh) = &self.bvh {
            bvh.intersect(ray)
        } else { Intersection::new() }
    }

    fn sample_light(&self) -> (Intersection, f32) {
        let emit_area_sum = self.objects.iter().fold(0.0, |acc, obj| {
            acc + if obj.has_emit() { obj.get_area() } else { 0.0 }
        });
        let p = get_random_float() * emit_area_sum;
        let mut emit_area_sum = 0.0;
        for object in self.objects.iter() {
            if !object.has_emit() { continue; }
            emit_area_sum += object.get_area();
            if p <= emit_area_sum {
                return object.sample();
            }
        }
        (Intersection::new(), 0.0)
    }

    pub fn cast_ray(&self, ray: &Ray) -> Vector3f {
        let obj_inter = self.intersect(ray);
        if !obj_inter.happened { return Vector3f::zeros(); }

        let m = obj_inter.m.as_ref().unwrap();
        if m.has_emission() { return m.get_emission().clone(); }

        let p = &obj_inter.coords;
        let normal = normalize(&obj_inter.normal);
        let wo = &ray.direction;
        let (light_point, pdf_l) = self.sample_light();
        let x = &light_point.coords;
        let ws = normalize(&(x - p));
        let light_ray = Ray::new(p.clone(), ws, 0.0);

        let (mut l_dir, mut l_indir) = (Vector3f::zeros(), Vector3f::zeros());
        let d = norm(&(p - light_point.coords));
        let light_inter = self.intersect(&light_ray);
        if light_inter.distance - d as f64 > -0.001 {
            let ev = m.eval(&wo, &light_ray.direction, &normal);
            let dot1 = dot(&light_ray.direction, &normal);
            let dot2 = dot(&light_ray.direction, &normalize(&(-light_point.normal)));
            l_dir = light_point.emit * ev * dot1 * dot2 / d.powi(2) / pdf_l;
        }

        if get_random_float() > self.russian_roulette { return l_dir; }

        let wi = normalize(&m.sample(&wo, &normal));
        let t_ray = Ray::new(p.clone(), wi.clone(), 0.0);
        let new_inter = self.intersect(&t_ray);
        if new_inter.happened && !new_inter.m.as_ref().unwrap().has_emission() {
            let shade = self.cast_ray(&t_ray);
            l_indir = shade * m.eval(&wo, &wi, &normal) * dot(&wi, &normal) /
                m.pdf(&wo, &wi, &normal) / self.russian_roulette;
        }

        l_dir + l_indir
    }
}
