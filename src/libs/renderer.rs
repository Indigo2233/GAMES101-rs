use std::fs::File;
use std::io::{BufWriter, Error, Write};
use super::global::{clamp, update_progress};
use super::ray::Ray;
use super::scene::Scene;
use super::vector::{normalize, Vector3f};

pub struct Renderer;

pub static EPSILON: f32 = 0.00001;

impl Renderer {
    pub fn render(scene: &Scene) -> Result<(), Error> {
        let mut framebuffer = vec!(Vector3f::zeros(); (scene.width * scene.height) as usize);
        let scale = (scene.fov * 0.5).to_radians().tan() as f32;
        let image_aspect_ratio = scene.width as f32 / scene.height as f32;
        let eye_pos = Vector3f::new(-1.0, 5.0, 10.0);
        let mut m = 0;
        for j in 0..scene.height {
            for i in 0..scene.width {
                let x = (2.0 * (i as f32 + 0.5) / scene.width as f32 - 1.0) * scale * image_aspect_ratio;
                let y = (1.0 - 2.0 * (j as f32 + 0.5) / scene.height as f32) * scale;

                let dir = normalize(&Vector3f::new(x, y, -1.0));
                let ray = Ray::new(eye_pos.clone(), dir, 0.0);
                framebuffer[m] = scene.cast_ray(&ray, scene, 0);
                m += 1;
            }
            update_progress(j as f64 / scene.height as f64);
        }
        update_progress(1.0);
        let mut file = BufWriter::new(File::create("binary.ppm")?);
        file.write_all(format!("P6\n{} {}\n255\n", scene.width, scene.height).as_bytes())?;
        let mut color = [0, 0, 0];
        for i in 0..scene.height * scene.width {
            color[0] = (255.0 * clamp(0.0, 1.0, framebuffer[i as usize].x)) as u8;
            color[1] = (255.0 * clamp(0.0, 1.0, framebuffer[i as usize].y)) as u8;
            color[2] = (255.0 * clamp(0.0, 1.0, framebuffer[i as usize].z)) as u8;
            file.write(&color).unwrap();
        }
        Ok(())
    }
}