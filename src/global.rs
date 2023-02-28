use std::io;
use std::io::Write;

pub fn clamp(lo: f32, hi: f32, v: f32) -> f32 {
    lo.max(hi.min(v))
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 || a == 0.0 { return None; } else if discr == 0.0 {
        return Some((-0.5 * b / a, -0.5 * b / a));
    }
    let q = if b > 0.0 {
        -0.5 * (b + discr.sqrt())
    } else {
        -0.5 * (b - discr.sqrt())
    };
    let x0 = q / a;
    let x1 = c / q;
    if x0 < x1 { Some((x0, x1)) } else { Some((x1, x0)) }
}

pub enum MaterialType {
    DiffuseAndGlossy,
    ReflectionAndRefraction,
    Reflection,
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
