use std::rc::Rc;
use std::thread::sleep;
use std::time::Instant;
use crate::libs::global::get_random_float;
use crate::libs::vector::norm;
use super::bounds3::{Axis, Bounds3};
use super::intersection::Intersection;
use super::object::Object;
use super::ray::Ray;

pub struct BVHBuildNode {
    bounds: Bounds3,
    left: Option<Rc<BVHBuildNode>>,
    right: Option<Rc<BVHBuildNode>>,
    object: Option<Rc<dyn Object>>,
    area: f32,
}

impl Default for BVHBuildNode {
    fn default() -> Self {
        BVHBuildNode {
            bounds: Default::default(),
            left: None,
            right: None,
            object: None,
            area: 0.0,
        }
    }
}

#[allow(dead_code)]
pub enum SplitMethod {
    Naive,
    SAH,
}

pub struct BVHAccel {
    pub root: Option<Rc<BVHBuildNode>>,
    pub max_prims_in_node: i32,
    pub split_method: SplitMethod,
    pub primitives: Vec<Rc<dyn Object>>,
}

impl BVHAccel {
    pub fn new(p: Vec<Rc<dyn Object>>, max_prims_in_node: i32, split_method: SplitMethod) -> Self {
        let start = Instant::now();
        let max_prims_in_node = 255.max(max_prims_in_node);

        let mut res = Self {
            root: None,
            max_prims_in_node,
            split_method,
            primitives: p,
        };
        if res.primitives.is_empty() { return res; }

        res.root = Self::recursive_build(&mut res.primitives);
        println!("\rBVH Generation complete: \nTime Taken: {:.2} secs\n\n", start.elapsed().as_secs_f32());
        res
    }
    pub fn default(p: Vec<Rc<dyn Object>>) -> Self {
        Self::new(p, 1, SplitMethod::Naive)
    }
    pub fn get_sample(node: Rc<BVHBuildNode>, p: f32) -> (Intersection, f32) {
        if node.left.is_none() || node.right.is_none() {
            let (pos, pdf) = node.object.as_ref().unwrap().sample();
            return (pos, pdf / node.area);
        }
        if p < node.left.as_ref().unwrap().area {
            Self::get_sample(node.left.clone().unwrap(), p)
        } else { Self::get_sample(node.right.clone().unwrap(), p - node.left.as_ref().unwrap().area) }
    }
    pub fn sample(&self) -> (Intersection, f32) {
        let p = get_random_float().sqrt() * self.root.as_ref().unwrap().area;
        let (pos, mut pdf) = Self::get_sample(self.root.clone().unwrap(), p);
        pdf /= self.root.as_ref().unwrap().area;
        (pos, pdf)
    }
    pub fn intersect(&self, ray: &Ray) -> Intersection {
        if self.root.is_none() { return Intersection::new(); }
        let root = self.root.clone().unwrap();
        BVHAccel::get_intersection(root, ray)
    }
    pub fn get_intersection(nodes: Rc<BVHBuildNode>, ray: &Ray) -> Intersection {
        let res = Intersection::new();
        if !nodes.bounds.intersect_p(ray, &ray.direction_inv,
                                     [ray.direction.x < 0.0, ray.direction.y < 0.0, ray.direction.z < 0.]) {
            return res;
        }
        if nodes.left.is_none() && nodes.right.is_none() {
            return match &nodes.object {
                None => res,
                Some(obj) => obj.get_intersection(ray.clone())
            };
        }
        let hit1 = BVHAccel::get_intersection(nodes.left.as_ref().unwrap().clone(), ray);
        let hit2 = BVHAccel::get_intersection(nodes.right.as_ref().unwrap().clone(), ray);
        if hit1.distance < hit2.distance { hit1 } else { hit2 }
    }

    pub fn recursive_build(objs: &mut Vec<Rc<dyn Object>>) -> Option<Rc<BVHBuildNode>> {
        let mut node = BVHBuildNode::default();
        let mut bounds = Bounds3::default();
        for i in 0..objs.len() {
            let obj_bound = objs[i].get_bounds();
            bounds = Bounds3::union_bounds(&bounds, &obj_bound);
        }
        if objs.len() == 1 {
            node.bounds = objs[0].get_bounds();
            node.object = Some(objs[0].clone());
            node.left = None;
            node.right = None;
            node.area = objs[0].get_area();
        } else if objs.len() == 2 {
            node.left = BVHAccel::recursive_build(&mut vec![objs[0].clone()]);
            node.right = BVHAccel::recursive_build(&mut vec![objs[1].clone()]);

            node.bounds = Bounds3::union_bounds(
                &node.left.as_ref().unwrap().bounds,
                &node.right.as_ref().unwrap().bounds,
            );
            node.area = node.left.as_ref().unwrap().area + node.right.as_ref().unwrap().area;
        } else {
            let centroid_bounds = objs.iter().fold(
                Bounds3::default(),
                |b, obj: &Rc<dyn Object>| { Bounds3::union_point(&b, &obj.get_bounds().centroid()) });
            let dim = centroid_bounds.max_extent();
            let half = objs.len() / 2;
            let (l, m, r) = match dim {
                Axis::X => objs.select_nth_unstable_by(
                    half, |o1, o2| { o1.get_bounds().centroid().x.partial_cmp(&o2.get_bounds().centroid().x).unwrap() }),
                Axis::Y => objs.select_nth_unstable_by(
                    half, |o1, o2| { o1.get_bounds().centroid().y.partial_cmp(&o2.get_bounds().centroid().y).unwrap() }),
                Axis::Z => objs.select_nth_unstable_by(
                    half, |o1, o2| { o1.get_bounds().centroid().z.partial_cmp(&o2.get_bounds().centroid().z).unwrap() }),
            };
            let mut left_shapes = l.to_vec();
            left_shapes.push(m.clone());
            let mut right_shapes = r.to_vec();
            let l = BVHAccel::recursive_build(&mut left_shapes);
            let r = BVHAccel::recursive_build(&mut right_shapes);
            node.left = l;
            node.right = r;
            node.bounds = Bounds3::union_bounds(&node.left.as_ref().unwrap().bounds,
                                                &node.right.as_ref().unwrap().bounds);
            node.area = node.left.as_ref().unwrap().area + node.right.as_ref().unwrap().area;
        }
        Some(Rc::new(node))
    }
}