// tests/utils.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;

    #[test]
    fn test_random_point_in_unit_sphere() {
        let num_samples = 100000;
        let mut sum = [0f64; 3];
        for _ in 0..num_samples {
            let p = random_point_in_unit_sphere();
            sum = sum.par_xor_sum(p);
        }
        let expected_mean = [0f64, 0f64, 0f64];
        for (a, b) in sum.par_iter().zip(expected_mean.iter()) {
            assert!((a - b).abs() < 0.01);
        }
    }

    #[test]
    fn test_random_point_on_sphere() {
        let num_samples = 100000;
        let mut sum = [0f64; 3];
        for _ in 0..num_samples {
            let p = random_point_on_sphere();
            sum = sum.par_xor_sum(p);
        }
        let expected_mean = [0f64, 0f64, 0f64];
        for (a, b) in sum.par_iter().zip(expected_mean.iter()) {
            assert!((a - b).abs() < 0.01);
        }
    }

    #[test]
    fn test_random_vector_in_unit_sphere() {
        let num_samples = 100000;
        let mut sum = [0f64; 3];
        for _ in 0..num_samples {
            let v = random_vector_in_unit_sphere();
            sum = sum.par_xor_sum(v);
        }
        let expected_mean = [0f64, 0f64, 0f64];
        for (a, b) in sum.par_iter().zip(expected_mean.iter()) {
            assert!((a - b).abs() < 0.01);
        }
    }
}

// Utility functions that don't depend on any specific part of the code
// These are used throughout the ray tracer

pub fn random_point_in_unit_sphere() -> [f64; 3] {
    let mut v = [0f64, 0f64, 0f64];
    loop {
        v = random_vector_in_unit_sphere();
        if dot(v, v) <= 1f64 {
            return v;
        }
    }
}

pub fn random_point_on_sphere(radius: f64) -> [f64; 3] {
    let mut v = [0f64, 0f64, 0f64];
    loop {
        v = random_vector_in_unit_sphere();
        if dot(v, v) <= 1f64 {
            return v.map(|x| x * radius);
        }
    }
}

pub fn random_vector_in_unit_sphere() -> [f64; 3] {
    let mut v = [0f64, 0f64, 0f64];
    v[0] = 2f64 * random_double() - 1f64;
    v[1] = 2f64 * random_double() - 1f64;
    v[2] = 2f64 * random_double() - 1f64;
    return v;
}

pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn random_double() -> f64 {
    thread_rng().gen::<f64>()
}

use rand::thread_rng;