use std::rc::Rc;
use crate::rope::Rope;
use crate::vector::Vector2d;

pub struct AppConfig {
    pub mass: f64,
    pub ks: f64,
    pub steps_per_frame: usize,
    pub gravity: Vector2d,
}

impl AppConfig {
    pub fn new() -> Self {
        Self { mass: 1.0, ks: 100.0, steps_per_frame: 64, gravity: Vector2d::new(0.0, -1.0) }
    }
}

pub struct Application {
    pub rope_euler: Option<Rc<Rope>>,
    pub rope_verlet: Option<Rc<Rope>>,
    pub screen_width: usize,
    pub screen_height: usize,
    pub config: AppConfig,
}

impl Application {
    pub fn new(config: AppConfig) -> Self {
        let rope_euler = Some(
            Rc::new(Rope::new(Vector2d::new(0.0, 200.0), Vector2d::new(-400.0, 200.0),
                              16, config.mass, config.ks, vec![0]))
        );
        let rope_verlet = Some(
            Rc::new(Rope::new(Vector2d::new(0.0, 200.0), Vector2d::new(-400.0, 200.0),
                              16, config.mass, config.ks, vec![0]))
        );

        Self {
            rope_euler,
            rope_verlet,
            screen_width: 960,
            screen_height: 960,
            config,
        }
    }

    pub fn update(&self) {
        for _ in 0..self.config.steps_per_frame {
            self.rope_euler.as_ref().
                unwrap().simulate_euler(1.0 / self.config.steps_per_frame as f64, self.config.gravity.clone());
            self.rope_verlet.as_ref().
                unwrap().simulate_verlet(1.0 / self.config.steps_per_frame as f64, self.config.gravity.clone());
        }
    }
}