use std::any::Any;
use std::rc::Rc;
use crate::bounds3::{Axis, Bounds3};
use crate::intersection::Intersection;
use crate::object::Object;
use crate::ray::Ray;
use crate::triangle::{MeshTriangle, Triangle};

pub struct BVHBuildNode<T: Object> {
    bounds: Bounds3,
    left: Option<Rc<BVHBuildNode<T>>>,
    right: Option<Rc<BVHBuildNode<T>>>,
    object: Option<Rc<T>>,
}

impl<T: Object> Default for BVHBuildNode<T> {
    fn default() -> Self {
        BVHBuildNode {
            bounds: Default::default(),
            left: None,
            right: None,
            object: None,
        }
    }
}

#[allow(dead_code)]
pub enum SplitMethod {
    Naive,
    SAH,
}

pub struct BVHAccel<T: Object> {
    pub root: Option<Rc<BVHBuildNode<T>>>,
    pub max_prims_in_node: i32,
    pub split_method: SplitMethod,
    pub primitives: Vec<Rc<T>>,
}

impl<T: Object + 'static> BVHAccel<T> {
    pub fn new(p: Vec<Rc<T>>, max_prims_in_node: i32, split_method: SplitMethod) -> Self {
        let max_prims_in_node = 255.max(max_prims_in_node);

        let mut res = Self {
            root: None,
            max_prims_in_node,
            split_method,
            primitives: p,
        };
        if res.primitives.is_empty() { return res; }

        res.root = Self::recursive_build(&mut res.primitives);
        res
    }
    pub fn default(p: Vec<Rc<T>>) -> Self {
        Self::new(p, 1, SplitMethod::Naive)
    }
    pub fn intersect(&self, ray: &Ray) -> Intersection {
        if self.root.is_none() { return Intersection::new(); }
        let root = self.root.clone().unwrap();
        BVHAccel::get_intersection(root, ray)
    }
    pub fn get_intersection(nodes: Rc<BVHBuildNode<T>>, ray: &Ray) -> Intersection {
        let res = Intersection::new();
        if !nodes.bounds.intersect_p(ray, &ray.direction_inv,
                                     [ray.direction.x < 0.0, ray.direction.y < 0.0, ray.direction.z < 0.]) {
            return res;
        }
        if nodes.left.is_none() && nodes.right.is_none() {
            return match &nodes.object {
                None => res,
                Some(obj) => {
                    let r = ray.clone();
                    obj.print_roll();
                    obj.get_intersection(r)
                }
            };
        }
        let hit1 = BVHAccel::get_intersection(nodes.left.as_ref().unwrap().clone(), ray);
        let hit2 = BVHAccel::get_intersection(nodes.right.as_ref().unwrap().clone(), ray);
        if hit1.distance < hit2.distance { hit1 } else { hit2 }
    }

    pub fn recursively_put(nodes: Rc<BVHBuildNode<T>>) {}
    pub fn recursive_build(objs: &mut Vec<Rc<T>>) -> Option<Rc<BVHBuildNode<T>>> {
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
        } else if objs.len() == 2 {
            node.left = BVHAccel::recursive_build(&mut vec![objs[0].clone()]);
            node.right = BVHAccel::recursive_build(&mut vec![objs[1].clone()]);

            node.bounds = Bounds3::union_bounds(
                &node.left.as_ref().unwrap().bounds,
                &node.right.as_ref().unwrap().bounds,
            );
        } else {
            let mut centroid_bounds = Bounds3::default();
            for i in 0..objs.len() {
                centroid_bounds = Bounds3::union_point(&centroid_bounds,
                                                       &objs[i].get_bounds().centroid());
            }
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
        }
        Some(Rc::new(node))
    }
}