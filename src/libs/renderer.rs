use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicIsize, Ordering};
use std::thread::spawn;
use super::global::{clamp, update_progress};
use super::ray::Ray;
use super::scene::Scene;
use super::vector::{normalize, Vector3f};

pub struct Renderer;

pub static EPSILON: f32 = 0.00001;

impl Renderer {
    pub fn render(scene: Scene) -> Result<(), Error> {
        let scene = Arc::new(scene);
        let framebuffer = vec!(Vector3f::zeros(); (scene.width * scene.height) as usize);
        let framebuffer = Arc::new(Mutex::new(framebuffer));
        let progress = Arc::new(AtomicIsize::new(0));
        let scale = (scene.fov * 0.5).to_radians().tan() as f32;
        let image_aspect_ratio = scene.width as f32 / scene.height as f32;
        let eye_pos = Vector3f::new(278.0, 273.0, -800.0);
        let spp = 128;
        let inv_spp = 1.0 / spp as f32;

        println!("SPP: {spp}");
        let (w, h) = (scene.width, scene.height);
        let n_threads = 16;
        let mut threads = vec![];

        let chunk_size = (h / n_threads) as usize;
        let rows: Vec<i32> = (0..h).collect();
        for chunk in rows.chunks(chunk_size) {
            let d = chunk.to_vec();
            let s = scene.clone();
            let buffer = framebuffer.clone();
            let pro = progress.clone();
            threads.push(spawn(move || {
                for j in d {
                    let mut res = vec!(Vector3f::zeros(); spp);
                    for i in 0..w {
                        let x = (2.0 * (i as f32 + 0.5) / w as f32 - 1.0) * scale * image_aspect_ratio;
                        let y = (1.0 - 2.0 * (j as f32 + 0.5) / h as f32) * scale;

                        let dir = normalize(&Vector3f::new(-x, y, 1.0));
                        let ray = Ray::new(eye_pos.clone(), dir, 0.0);
                        for k in 0..spp { res[k] = s.cast_ray(&ray, 0) * inv_spp; }
                        let mut fb = buffer.lock().unwrap();
                        let m = j * w + i;
                        fb[m as usize] = res.iter().fold(Vector3f::zeros(), |cur, r| cur + r);
                    }
                    pro.fetch_add(1, Ordering::SeqCst);
                    update_progress(pro.load(Ordering::SeqCst) as f64 / h as f64);
                }
            }));
        }
        for thread in threads { thread.join().unwrap(); }
        update_progress(1.0);

        let mut file = BufWriter::new(File::create("binary.ppm")?);
        file.write_all(format!("P6\n{} {}\n255\n", w, h).as_bytes())?;
        let mut color = [0, 0, 0];
        let fb = framebuffer.lock().unwrap();
        for i in 0..h * w {
            color[0] = (255.0 * clamp(0.0, 1.0, fb[i as usize].x).powf(0.6)) as u8;
            color[1] = (255.0 * clamp(0.0, 1.0, fb[i as usize].y).powf(0.6)) as u8;
            color[2] = (255.0 * clamp(0.0, 1.0, fb[i as usize].z).powf(0.6)) as u8;
            file.write(&color).unwrap();
        }
        Ok(())
    }
}