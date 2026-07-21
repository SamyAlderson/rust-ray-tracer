// src/main.rs
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use nalgebra::Vector3;
use rayon::prelude::*;

mod bvh;
mod camera;
mod lighting;
mod materials;
mod objects;
mod utils;

fn main() -> Result<(), String> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Usage: cargo run <output_file> <scene_file>".to_string());
    }
    let output_file = &args[1];
    let scene_file = &args[2];

    // Load scene file
    if !Path::new(scene_file).exists() {
        return Err(format!("Error: Scene file '{}' not found", scene_file));
    }

    // Create image writer
    let output_path = Path::new(output_file);
    let output_dir = output_path.parent().unwrap();
    std::fs::create_dir_all(output_dir)?;
    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);

    // Render image
    let image = render_scene(scene_file, writer)?;

    // Save image
    image.save(writer).map_err(|e| format!("Error saving image: {}", e))?;

    Ok(())
}

fn render_scene(scene_file: &str, writer: impl Write) -> Result<image::DynamicImage, String> {
    // Load scene from file
    let scene = objects::load_scene(scene_file)?;

    // Create camera
    let camera = camera::create_camera(&scene.camera)?;

    // Create BVH acceleration structure
    let bvh = bvh::create_bvh(&scene.objects)?;

    // Render image
    let image = utils::render_image(
        &camera,
        &scene.lights,
        &bvh,
        &scene.materials,
        &scene.objects,
    )?;

    // Save image to writer
    writer.write_all(image.as_pixel_data())?;

    Ok(image)
}