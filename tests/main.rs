// tests/main.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{objects::Sphere, materials::Lambertian, camera::Camera};
    use nalgebra::Vector3;

    #[test]
    fn test_sphere_intersection() {
        let sphere = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let intersections = sphere.intersect(&ray);
        assert!(intersections.is_some());
    }

    #[test]
    fn test_lambertian_color() {
        let material = Lambertian::new(Vector3::new(0.5, 0.5, 0.5));
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let color = material.color(&ray);
        assert!(color.is_some());
    }

    #[test]
    fn test_camera_ray() {
        let camera = Camera::new();
        let ray = camera.get_ray(50.0, 50.0);
        assert!(ray.is_some());
    }
}

// tests/utils.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::random_float;

    #[test]
    fn test_random_float() {
        let value = random_float(0.0, 1.0);
        assert!(value >= 0.0 && value <= 1.0);
    }
}