use std::io;
use std::io::Write;

pub fn clamp(lo: f32, hi: f32, v: f32) -> f32 {
    lo.max(hi.min(v))
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MaterialType {
    Diffuse,
}

pub fn update_progress(progress: f64) {
    let bar_width = 70;
    print!("[");
    let pos = bar_width as f64 * progress;
    for i in 0..bar_width {
        if i < pos as i32 {
            print!("=");
        } else if i == pos as i32 { print!(">"); } else { print!(" "); }
    }
    print!("] {} %", (progress * 100.0) as i32);
    io::stdout().flush().unwrap();
    print!("\r");
}

pub fn get_random_float() -> f32 {
    // rand::random::<f32>()
    0.2
}