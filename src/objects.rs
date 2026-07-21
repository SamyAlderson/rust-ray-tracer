// Implementation of Scene objects
// This module contains the definition of objects in the scene, such as Sphere, Plane, and Mesh

use nalgebra::Point3;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use self::materials::Material;
use self::camera::{Camera, Ray};
use super::utils::{BoundingBox, intersect_ray_bounding_box, intersect_ray_sphere};

pub enum Object {
    Sphere {
        center: Point3<f64>,
        radius: f64,
        material: Material,
    },
    Plane {
        point: Point3<f64>,
        normal: Point3<f64>,
        material: Material,
    },
    Mesh {
        vertices: Vec<Point3<f64>>,
        faces: Vec<Vec<usize>>,
        material: Material,
    },
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Object::Sphere {
                center, radius, material,
            } => intersect_ray_sphere(ray, center, radius),
            Object::Plane {
                point, normal, material,
            } => intersect_ray_bounding_box(ray, BoundingBox::new(*point, *point + *normal)),
            Object::Mesh {
                vertices, faces, material,
            } => {
                // Approximate the mesh as a bounding box
                let bbox = BoundingBox::from_vertices(vertices);
                intersect_ray_bounding_box(ray, bbox).map(|t| t.min(1.0).max(0.0))
            }
        }
    }

    pub fn normal(&self, p: &Point3<f64>) -> Option<Point3<f64>> {
        match self {
            Object::Sphere { .. } => None,
            Object::Plane { point, normal, .. } => Some(*normal),
            Object::Mesh { .. } => {
                let mut intersection = self.intersect(&Ray::new(*p, *normal));
                if intersection.is_some() {
                    Some(normal)
                } else {
                    None
                }
            }
        }
    }
}

pub trait ObjectIterator {
    type Item;
    fn iter(&self) -> Self::Item;
}

impl<'a> ObjectIterator for &'a Object {
    type Item = &'a Object;
    fn iter(&self) -> Self::Item {
        self
    }
}

impl<'a> ObjectIterator for &[Object] {
    type Item = &'a Object;
    fn iter(&self) -> Self::Item {
        self
    }
}

impl<'a> ObjectIterator for ParIterator<'a, Object> {
    type Item = ParIterator<'a, Object>;
    fn iter(&self) -> Self::Item {
        self.par_iter()
    }
}

impl<'a> ObjectIterator for Object {
    type Item = Object;
    fn iter(&self) -> Self::Item {
        self
    }
}