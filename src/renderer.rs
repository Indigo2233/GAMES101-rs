use std::fs::File;
use std::io::Write;
use std::mem::swap;
use std::rc::Rc;
use crate::global::{clamp, MaterialType, update_progress};
use crate::object::Object;
use crate::scene::Scene;
use crate::vector::{dot, normalize, Vector2f, Vector3f};

struct HitPayload {
    pub t_near: f32,
    pub index: usize,
    pub uv: Vector2f,
    pub hit_obj: Rc<dyn Object>,
}

static K_INF: f32 = f32::MAX;

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
        n0 = -n;
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
    // println!("{cosi},{etai},{etat},{sint}");
    if sint >= 1.0 { 1.0 } else {
        let cost = (0.0_f32).max(1.0 - sint * sint).sqrt();
        cosi = cosi.abs();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
        (rs * rs + rp * rp) / 2.0
    }
}

fn trace(orig: &Vector3f, dir: &Vector3f, objs: &Vec<Rc<dyn Object>>) -> Option<HitPayload> {
    let mut t_near = K_INF;
    let mut payload: Option<HitPayload> = None;
    for obj in objs.iter() {
        if let Some((t_near_k, index_k, uv_k)) = obj.intersect(&orig, &dir) {
            if t_near > t_near_k {
                payload = Some(HitPayload {
                    t_near: t_near_k,
                    index: index_k,
                    uv: uv_k,
                    hit_obj: obj.clone(),
                });
                t_near = t_near_k;
            }
        }
    }
    payload
}

fn cast_ray(orig: &Vector3f, dir: &Vector3f, scene: &Scene, depth: i32) -> Vector3f {
    if depth > scene.max_depth { return Vector3f::zeros(); }
    let mut hit_color = scene.background_color.clone();
    if let Some(payload) = trace(orig, dir, scene.objects()) {
        let hit_point = dir * payload.t_near + orig;
        let mut st = Vector2f::zeros();
        let normal = payload.hit_obj.get_surface_properties(&hit_point, dir, payload.index, payload.uv, &mut st);

        match &payload.hit_obj.attribute().material_type {
            MaterialType::ReflectionAndRefraction => {
                // if depth == 0 {
                //     println!("{:?}", normal);
                // }
                let reflection_dir = normalize(&reflect(dir, &normal));
                let refraction_dir = normalize(&refract(dir, &normal, payload.hit_obj.attribute().ior));
                let reflection_ray_orig = if dot(&reflection_dir, &normal) < 0.0 {
                    &hit_point - &normal * scene.epsilon
                } else { &hit_point + &normal * scene.epsilon };
                let refraction_ray_orig = if dot(&refraction_dir, &normal) < 0.0 {
                    &hit_point - &normal * scene.epsilon
                } else { &hit_point + &normal * scene.epsilon };
                let reflection_color = cast_ray(&reflection_ray_orig, &reflection_dir, scene, depth + 1);
                let refraction_color = cast_ray(&refraction_ray_orig, &refraction_dir, scene, depth + 1);
                let kr = fresnel(dir, &normal, payload.hit_obj.attribute().ior);

                hit_color = reflection_color * kr + refraction_color * (1.0 - kr);
            }

            MaterialType::Reflection => {
                let kr = fresnel(dir, &normal, payload.hit_obj.attribute().ior);
                let reflection_dir = &reflect(dir, &normal);
                let reflection_ray_orig = if dot(&reflection_dir, &normal) < 0.0 {
                    &hit_point + &normal * scene.epsilon
                } else { &hit_point - &normal * scene.epsilon };
                hit_color = cast_ray(&reflection_ray_orig, reflection_dir, scene, depth + 1) * kr;
            }
            MaterialType::DiffuseAndGlossy => {
                let mut light_amt = Vector3f::zeros();
                let mut specular_color = Vector3f::zeros();
                let shadow_point_orig = if dot(dir, &normal) < 0.0 {
                    &hit_point + &normal * scene.epsilon
                } else { &hit_point - &normal * scene.epsilon };
                for light in scene.lights() {
                    let mut light_dir = &light.position - &hit_point;
                    let light_dist2 = dot(&light_dir, &light_dir);
                    light_dir = normalize(&light_dir);
                    let l_dot_n = (0.0_f32).max(dot(&light_dir, &normal));
                    let shadow_res = trace(&shadow_point_orig, &light_dir, scene.objects());
                    let in_shadow = match shadow_res {
                        None => false,
                        Some(res) => res.t_near * res.t_near < light_dist2
                    };
                    light_amt += if in_shadow { Vector3f::zeros() } else { &light.intensity * l_dot_n };
                    let reflection_dir = reflect(&(-light_dir), &normal);

                    specular_color += ((0.0_f32).max(-dot(&reflection_dir, dir))).
                        powf(payload.hit_obj.attribute().specular_exponent) * &light.intensity;
                }
                hit_color = &light_amt * payload.hit_obj.eval_diffuse_color(&st) * payload.hit_obj.attribute().kd +
                    specular_color * payload.hit_obj.attribute().ks;
            }
        }
    }

    hit_color
}

pub struct Renderer;

impl Renderer {
    pub fn render(scene: &Scene) {
        let mut framebuffer = vec!(Vector3f::new(0.6, 0.7, 0.9); (scene.width * scene.height) as usize);
        let scale = (scene.fov * 0.5).to_radians().tan() as f32;
        let image_aspect_ratio = scene.width as f32 / scene.height as f32;
        let eye_pos = Vector3f::same(0.0);
        let mut m = 0;
        for j in 0..scene.height {
            for i in 0..scene.width {
                let x = 2.0 * scale * image_aspect_ratio / scene.width as f32 * (i as f32 + 0.5)
                    - scale * image_aspect_ratio;
                let y = -2.0 * scale / scene.height as f32 * (j as f32 + 0.5) + scale;
                let dir = normalize(&Vector3f::new(x, y, -1.0));
                framebuffer[m] = cast_ray(&eye_pos, &dir, scene, 0);
                m += 1;
            }
            update_progress(j as f64 / scene.height as f64);
        }
        let mut file = File::create("binary.ppm").unwrap();
        file.write_all(format!("P6\n{} {}\n255\n", scene.width, scene.height).as_bytes()).unwrap();
        let mut color = [0u8, 0, 0];
        for i in 0..scene.height * scene.width {
            color[0] = (255.0 * clamp(0.0, 1.0, framebuffer[i as usize].x)) as u8;
            color[1] = (255.0 * clamp(0.0, 1.0, framebuffer[i as usize].y)) as u8;
            color[2] = (255.0 * clamp(0.0, 1.0, framebuffer[i as usize].z)) as u8;
            file.write_all(&color).unwrap();
        }
    }
}