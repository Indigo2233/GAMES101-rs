use std::mem::swap;
use std::ops::Neg;
use std::rc::Rc;
use crate::bvh::{BVHAccel, SplitMethod};
use crate::global::{clamp, MaterialType};
use crate::light::Light;
use crate::object::Object;
use crate::ray::Ray;
use crate::vector::{dot, normalize, Vector2f, Vector3f};
use crate::intersection::Intersection;
use crate::renderer::EPSILON;


static K_INF: f32 = f32::MAX;

pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub fov: f64,
    pub background_color: Vector3f,
    pub max_depth: i32,
    objects: Vec<Rc<dyn Object>>,
    lights: Vec<Box<Light>>,
    bvh: Option<Rc<BVHAccel>>,
}

impl Scene {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            fov: 90.0,
            background_color: Vector3f::new(0.235294, 0.67451, 0.843137),
            max_depth: 5,
            objects: vec![],
            lights: vec![],
            bvh: None,
        }
    }
    pub fn add_obj(&mut self, object: Rc<dyn Object>) {
        self.objects.push(object);
    }
    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }

    pub fn objects(&self) -> &Vec<Rc<dyn Object>> {
        &self.objects
    }
    pub fn lights(&self) -> &Vec<Box<Light>> {
        &self.lights
    }

    pub fn build_bvh(&mut self) {
        println!(" - Generating BVH...\n");
        let objs = self.objects.clone();
        self.bvh = Some(Rc::new(BVHAccel::new(objs, 1, SplitMethod::Naive)));
    }
    pub fn intersect(&self, ray: &Ray) -> Intersection {
        if let Some(bvh) = &self.bvh {
            bvh.intersect(ray)
        } else { Intersection::new() }
    }
    pub fn cast_ray(&self, ray: &Ray, scene: &Scene, depth: i32) -> Vector3f {
        if depth > scene.max_depth { return Vector3f::zeros(); }
        let intersection = self.intersect(ray);
        let m = intersection.m.as_ref();
        let hit_obj = &intersection.obj.as_ref();
        let mut hit_color = scene.background_color.clone();
        if intersection.happened {
            let hit_point = intersection.coords;
            let mut st = Vector2f::zeros();
            let normal = hit_obj.unwrap().get_surface_properties(&hit_point, &ray.direction, 0, Vector2f::zeros(), &mut st);
            match m.unwrap().material_type {
                MaterialType::DiffuseAndGlossy => {
                    let reflection_dir = normalize(&reflect(&ray.direction, &normal));
                    let refraction_dir = normalize(&refract(&ray.direction, &normal, m.unwrap().ior));
                    let reflection_ray_orig = if dot(&reflection_dir, &normal) < 0.0 {
                        &hit_point - &normal * EPSILON
                    } else { &hit_point + &normal * EPSILON };
                    let refraction_ray_orig = if dot(&refraction_dir, &normal) < 0.0 {
                        &hit_point - &normal * EPSILON
                    } else { &hit_point + &normal * EPSILON };
                    let r1 = Ray::new(reflection_ray_orig, reflection_dir, 0.0);
                    let r2 = Ray::new(refraction_ray_orig, refraction_dir, 0.0);
                    let reflection_color = scene.cast_ray(&r1, scene, depth + 1);
                    let refraction_color = scene.cast_ray(&r2, scene, depth + 1);
                    let kr = fresnel(&ray.direction, &normal, m.unwrap().ior);
                    hit_color = reflection_color * kr + refraction_color * (1.0 - kr);
                }
                MaterialType::ReflectionAndRefraction => {
                    let kr = fresnel(&ray.direction, &normal, m.unwrap().ior);
                    let reflection_dir = reflect(&ray.direction, &normal);
                    let reflection_ray_orig = if dot(&reflection_dir, &normal) < 0.0 {
                        &hit_point + &normal * EPSILON
                    } else { &hit_point - &normal * EPSILON };
                    let ray = Ray::new(reflection_ray_orig, reflection_dir, 0.0);
                    hit_color = self.cast_ray(&ray, scene, depth + 1) * kr;
                }
                MaterialType::Reflection => {
                    let mut light_amt = Vector3f::zeros();
                    let mut specular_color = Vector3f::zeros();
                    let shadow_point_orig = if dot(&ray.direction, &normal) < 0.0 {
                        &hit_point + &normal * EPSILON
                    } else { &hit_point - &normal * EPSILON };
                    for light in scene.lights() {
                        let mut light_dir = &light.position - &hit_point;
                        let light_dist2 = dot(&light_dir, &light_dir);
                        light_dir = normalize(&light_dir);
                        let l_dot_n = (0.0_f32).max(dot(&light_dir, &normal));
                        let ray = Ray::new(shadow_point_orig.clone(), light_dir.clone(), 0.0);
                        let in_shadow = self.bvh.as_ref().unwrap().intersect(&ray).happened;
                        light_amt += if in_shadow { Vector3f::zeros() } else { &light.intensity * l_dot_n };
                        let reflection_dir = reflect(&(-light_dir), &normal);

                        specular_color += ((0.0_f32).max(-dot(&reflection_dir, &ray.direction))).
                            powf(m.unwrap().specular_exponent) * &light.intensity;
                    }
                    hit_color = &light_amt * (hit_obj.unwrap().eval_diffuse_color(&st) * m.unwrap().kd +
                        specular_color * m.unwrap().ks);
                }
            }
        }
        hit_color
    }
}

fn reflect(i: &Vector3f, n: &Vector3f) -> Vector3f {
    i - 2.0 * dot(i, n) * n
}

fn refract(i: &Vector3f, n: &Vector3f, ior: f32) -> Vector3f {
    let mut cosi = clamp(-1.0, 1.0, dot(i, n));
    let mut etai = 1.0;
    let mut etat = ior;
    let mut n0 = n.clone();
    if cosi < 0.0 { cosi = -cosi; } else {
        swap(&mut etai, &mut etat);
        n0 = n.neg();
    }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 { Vector3f::zeros() } else {
        eta * i + (eta * cosi - k.sqrt()) * n0
    }
}

fn fresnel(i: &Vector3f, n: &Vector3f, ior: f32) -> f32 {
    let mut cosi = clamp(-1.0, 1.0, dot(i, n));
    let mut etai = 1.0;
    let mut etat = ior;
    if cosi > 0.0 { swap(&mut etai, &mut etat); }
    let sint = etai / etat * (0.0_f32).max(1.0 - cosi * cosi).sqrt();
    if sint >= 1.0 { 1.0 } else {
        let cost = (0.0_f32).max(1.0 - sint * sint).sqrt();
        cosi = cosi.abs();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
        (rs * rs + rp * rp) / 2.0
    }
}