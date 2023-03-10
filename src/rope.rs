use std::cell::RefCell;
use std::rc::Rc;
use super::vector::Vector2d;

pub struct Mass {
    pub mass: f64,
    pub pinned: bool,
    pub start_position: Vector2d,
    pub position: Vector2d,
    pub last_position: Vector2d,
    pub velocity: Vector2d,
    pub force: Vector2d,
}

impl Mass {
    pub fn new(mass: f64, pinned: bool, start_position: Vector2d) -> Self {
        Self {
            mass,
            pinned,
            start_position,
            position: start_position,
            last_position: start_position,
            velocity: Vector2d::zeros(),
            force: Vector2d::zeros(),
        }
    }
}

pub struct Spring {
    pub k: f64,
    pub rest_len: f64,
    pub m1: Rc<RefCell<Mass>>,
    pub m2: Rc<RefCell<Mass>>,
}

impl Spring {
    pub fn new(k: f64, m1: Rc<RefCell<Mass>>, m2: Rc<RefCell<Mass>>) -> Self {
        let rest_len = (m1.borrow().position - m2.borrow().position).norm();
        Self { k, rest_len, m1, m2 }
    }
}

pub struct Rope {
    pub masses: Vec<Rc<RefCell<Mass>>>,
    pub springs: Vec<Rc<RefCell<Spring>>>,
}

impl Rope {
    pub fn new(start: Vector2d, end: Vector2d, num_nodes: i32, node_mass: f64, k: f64,
               pinned_nodes: Vec<usize>) -> Self {
        let delta = (end - start) / (num_nodes - 1) as f64;
        let masses: Vec<Rc<RefCell<Mass>>> = (0..num_nodes).map(|i|
            Rc::new(RefCell::new(
                Mass::new(node_mass, false, start + delta * i as f64)))).collect();
        let springs: Vec<Rc<RefCell<Spring>>> = (0..(num_nodes - 1) as usize).map(|i| {
            Rc::new(RefCell::new(
                Spring::new(k, masses[i].clone(), masses[i + 1].clone())))
        }).collect();
        for i in pinned_nodes { masses[i].borrow_mut().pinned = true; }
        Self { masses, springs }
    }

    pub fn simulate_euler(&self, delta_t: f64, gravity: Vector2d) {
        for s in self.springs.iter() {
            let s = s.borrow();
            let m1 = &s.m1;
            let m2 = &s.m2;
            let a2b = m2.borrow().position - m1.borrow().position;
            let f = -s.k * a2b.unit() * (a2b.norm() - s.rest_len);
            m1.borrow_mut().force -= f;
            m2.borrow_mut().force += f;
        }
        for m in self.masses.iter() {
            let mut m = m.borrow_mut();
            if m.pinned {
                m.force = Vector2d::zeros();
                continue;
            }
            let mut a = m.force / m.mass;
            let f_d = -0.01 * m.velocity;
            a += gravity + f_d / m.mass;
            m.velocity += a * delta_t;
            let delta_p = m.velocity * delta_t;
            m.position += delta_p;
            m.force = Vector2d::zeros();
        }
    }
    pub fn simulate_verlet(&self, delta_t: f64, gravity: Vector2d) {
        for s in self.springs.iter() {
            let s = s.borrow();
            let mut m1 = s.m1.borrow_mut();
            let mut m2 = s.m2.borrow_mut();
            let b2a = m2.position - m1.position;
            let dist = b2a.norm();
            let dir_ba = b2a / dist;
            if m1.pinned {
                if m2.pinned { continue; }
                m2.position += -dir_ba * (dist - s.rest_len);
            } else {
                if m2.pinned {
                    m1.position += dir_ba * (dist - s.rest_len);
                } else {
                    m1.position += dir_ba * (dist - s.rest_len) * 0.5;
                    m2.position += -dir_ba * (dist - s.rest_len) * 0.5;
                }
            }
        }
        for m in self.masses.iter() {
            let mut m = m.borrow_mut();
            if m.pinned { continue;}
            let a = gravity;
            let v_old = m.position;
            let damping = 0.00005;
            let dp = (1.0 - damping) * (m.position - m.last_position);
            m.position += dp + a * delta_t * delta_t;
            m.last_position = v_old;
        }
    }
}