//! BVH acceleration structure implementation.

use nalgebra::Vector3;
use rayon::prelude::*;
use std::iter;

/// Axis-aligned bounding box.
struct AABB {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl AABB {
    /// Create a new AABB from a list of points.
    fn from_points(points: &[Vector3<f32>]) -> Self {
        let min = points.iter().map(|p| p.x).min().unwrap()..=points.iter().map(|p| p.y).min().unwrap();
        let max = points.iter().map(|p| p.x).max().unwrap()..=points.iter().map(|p| p.y).max().unwrap();
        let z_min = points.iter().map(|p| p.z).min().unwrap();
        let z_max = points.iter().map(|p| p.z).max().unwrap();
        AABB {
            min: Vector3::new(min.0, min.1, z_min),
            max: Vector3::new(max.0, max.1, z_max),
        }
    }

    /// Check if a ray intersects with the AABB.
    fn intersect(&self, ray_dir: &Vector3<f32>, ray_origin: &Vector3<f32>) -> bool {
        let t_min = (self.min - ray_origin) / *ray_dir;
        let t_max = (self.max - ray_origin) / *ray_dir;
        t_min.x <= t_max.x && t_min.y <= t_max.y && t_min.z <= t_max.z
    }
}

/// BVH node.
struct Node {
    aabb: AABB,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Create a new BVH node.
    fn new(aabb: AABB, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node { aabb, left, right }
    }

    /// Split the node into two children.
    fn split(&self, points: &[Vector3<f32>]) -> (Vec<Box<Node>>, Vec<Box<Node>>) {
        let (mut left_points, mut right_points) = points.iter().map(|p| (p.x, p.y)).partition(|&(x, y)| x < self.aabb.min.x);
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut queue = vec![(self.aabb.min, self.aabb.max)];
        while let Some((min, max)) = queue.pop() {
            for i in (min.x..=max.x).zip((min.y..=max.y)) {
                if let Some(point) = points.iter().find(|p| p.x == i.0 && p.y == i.1) {
                    if left_points.remove(point) {
                        left.push(Node::new(AABB::from_points(&left_points), None, None));
                        queue.push((min, Vector3::new(i.0 + 1, i.1, max.z)));
                    } else {
                        right.push(Node::new(AABB::from_points(&right_points), None, None));
                        queue.push((Vector3::new(min.x + 1, min.y, max.z), max));
                    }
                }
            }
        }
        (left, right)
    }

    /// Intersect the ray with the node.
    fn intersect(&self, ray_dir: &Vector3<f32>, ray_origin: &Vector3<f32>) -> Option<AABB> {
        if self.aabb.intersect(ray_dir, ray_origin) {
            if let Some(left) = &self.left {
                if let Some(intersect) = left.intersect(ray_dir, ray_origin) {
                    return Some(intersect);
                }
            }
            if let Some(right) = &self.right {
                if let Some(intersect) = right.intersect(ray_dir, ray_origin) {
                    return Some(intersect);
                }
            }
            Some(self.aabb.clone())
        } else {
            None
        }
    }
}

/// BVH acceleration structure.
pub struct BVH {
    root: Option<Box<Node>>,
}

impl BVH {
    /// Create a new BVH from a list of points.
    pub fn new(points: &[Vector3<f32>]) -> Self {
        let (left, right) = points.iter().map(|p| (*p.x, *p.y)).partition(|&(x, y)| x < 0.0);
        let (min, max) = (left.iter().map(|&(x, y)| x).min().unwrap(), left.iter().map(|&(x, y)| x).max().unwrap());
        let mut queue = vec![(min, max)];
        while let Some((min, max)) = queue.pop() {
            let (left, right) = points.iter().map(|p| (*p.x, *p.y)).partition(|&(x, y)| x < min);
            queue.push((min, (min + max) / 2.0));
            queue.push(((min + max) / 2.0, max));
            if !left.iter().all(|&(x, y)| x < min) {
                let node = Node::new(AABB::from_points(&left), None, None);
                if let Some(right) = &mut self.root {
                    *right = Some(Box::new(node));
                } else {
                    self.root = Some(Box::new(node));
                }
            }
            if !right.iter().all(|&(x, y)| x < (min + max) / 2.0) {
                let node = Node::new(AABB::from_points(&right), None, None);
                if let Some(left) = &mut self.root {
                    left.left = Some(Box::new(node));
                } else {
                    self.root = Some(Box::new(node));
                }
            }
        }
        BVH { root: self.root }
    }

    /// Intersect the ray with the BVH.
    pub fn intersect(&self, ray_dir: &Vector3<f32>, ray_origin: &Vector3<f32>) -> Option<AABB> {
        if let Some(node) = &self.root {
            node.intersect(ray_dir, ray_origin)
        } else {
            None
        }
    }
}