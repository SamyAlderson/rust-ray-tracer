// Benchmarking functionality for the ray tracer

use rayon::prelude::*;
use nalgebra::Vector3;
use image::{DynamicImage, GenericImageView};
use std::time::Instant;
use std::sync::Arc;

// Benchmark types
enum Benchmark {
    RayTracing,
    BVHConstruction,
}

// Benchmark result
struct Result {
    duration: f64,
    error: Option<String>,
}

impl Result {
    fn new(duration: f64, error: Option<String>) -> Self {
        Self { duration, error }
    }
}

// Benchmark trait
trait Benchmarkable {
    fn benchmark(&self, benchmark: Benchmark) -> Result;
}

// Ray tracer implementation
struct RayTracer {
    scene: Arc<Vec<crate::Object>>,
    camera: Arc<crate::Camera>,
}

impl RayTracer {
    fn new(scene: Arc<Vec<crate::Object>>, camera: Arc<crate::Camera>) -> Self {
        Self { scene, camera }
    }
}

impl Benchmarkable for RayTracer {
    fn benchmark(&self, benchmark: Benchmark) -> Result {
        let start = Instant::now();
        match benchmark {
            Benchmark::RayTracing => self.ray_trace(),
            Benchmark::BVHConstruction => self.construct_bvh(),
        }
        let duration = start.elapsed().as_secs_f64();
        if let Some(error) = self.error() {
            Result::new(duration, Some(error))
        } else {
            Result::new(duration, None)
        }
    }

    fn error(&self) -> Option<String> {
        None
    }
}

impl rayon::ParallelSliceMut for Vec<crate::Object> {
    fn par_iter_mut(&mut self) -> rayon::IntoParallelIterator<Self> {
        self.par_iter_mut()
    }
}

// BVH construction
fn construct_bvh(scene: Arc<Vec<crate::Object>>) -> Result {
    let start = Instant::now();
    let bvh = crate::BVH::new(scene);
    let duration = start.elapsed().as_secs_f64();
    Result::new(duration, None)
}

// Ray tracing
fn ray_trace(ray_tracer: &RayTracer) -> Result {
    let start = Instant::now();
    let pixels = ray_tracer.ray_trace();
    let duration = start.elapsed().as_secs_f64();
    Result::new(duration, None)
}

// Image generation
fn generate_image(pixels: Vec<crate::Color>) -> DynamicImage {
    let (width, height) = (800, 600);
    let mut image = DynamicImage::new_rgb8(width, height);
    for (i, pixel) in pixels.iter().enumerate() {
        let x = i % width;
        let y = i / width;
        image.put_pixel(x, y, *pixel);
    }
    image
}

// Main benchmark function
fn main() {
    let scene = Arc::new(crate::Scene::new());
    let camera = Arc::new(crate::Camera::new());
    let ray_tracer = RayTracer::new(scene.clone(), camera.clone());
    let benchmark = Benchmark::RayTracing;
    let result = ray_tracer.benchmark(benchmark);
    if let Some(error) = result.error {
        eprintln!("Error: {}", error);
    } else {
        println!("Duration: {}", result.duration);
        let pixels = ray_tracer.ray_trace();
        let image = generate_image(pixels);
        image.save("output.png").unwrap();
    }
}