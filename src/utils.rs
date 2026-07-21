// src/utils.rs

/// Calculates the dot product of two vectors.
///
/// This function is used extensively throughout the project, so it's worth optimizing.
/// We use the `nalgebra` crate for vector operations.
pub fn dot_product(a: &nalgebra::Vector3<f64>, b: &nalgebra::Vector3<f64>) -> f64 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

/// Calculates the magnitude of a vector.
///
/// This is a straightforward implementation, but it's worth noting that we're using
/// the `sqrt` function from the standard library, which is not the most efficient.
/// However, this function is only called rarely, so it's not a major bottleneck.
pub fn magnitude(v: &nalgebra::Vector3<f64>) -> f64 {
    v.norm()
}

/// Checks if two vectors are nearly equal.
///
/// This is a common idiom in graphics programming: two vectors are considered equal
/// if they're within a certain tolerance of each other.
pub fn nearly_equal(a: &nalgebra::Vector3<f64>, b: &nalgebra::Vector3<f64>, epsilon: f64) -> bool {
    let dot = dot_product(a, b);
    let magnitude_a = magnitude(a);
    let magnitude_b = magnitude(b);
    let dot_product_magnitude = magnitude_a * magnitude_b;
    (dot >= dot_product_magnitude - epsilon) && (dot <= dot_product_magnitude + epsilon)
}

/// Calculates the intersection point of two rays.
///
/// This function is used to determine when a ray intersects with an object in the scene.
/// It's a bit involved, but the math is straightforward.
pub fn ray_intersection(ray_origin: &nalgebra::Vector3<f64>, ray_direction: &nalgebra::Vector3<f64>, object_center: &nalgebra::Vector3<f64>, radius: f64) -> Option<nalgebra::Vector3<f64>> {
    let l = object_center - ray_origin;
    let tca = dot_product(&l, ray_direction);
    if tca < 0.0 {
        return None;
    }
    let d2 = dot_product(&l, &l) - tca * tca;
    if d2 > radius * radius {
        return None;
    }
    Some(ray_origin + (tca * ray_direction) + (l - tca * ray_direction).normalize().scale(radius))
}

/// Checks if a point is inside a sphere.
///
/// This function is used to determine if an intersection point is inside an object.
/// It's a simple implementation, but it's worth noting that we're using the `normalize`
/// method from the `nalgebra` crate to normalize the vector.
pub fn point_inside_sphere(point: &nalgebra::Vector3<f64>, center: &nalgebra::Vector3<f64>, radius: f64) -> bool {
    magnitude(point - center) <= radius
}