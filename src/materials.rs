// Material properties implementation
// This module defines the material properties used in the ray tracing simulation
// Materials are responsible for determining how light interacts with scene objects

// Import necessary dependencies
use nalgebra as na;
use std::f64::consts::PI;

// Define the Material trait
pub trait Material {
    // Get the diffuse color of the material
    fn diffuse_color(&self) -> [f64; 3];
    // Get the specular color of the material
    fn specular_color(&self) -> [f64; 3];
    // Get the refractive index of the material
    fn refractive_index(&self) -> f64;
    // Get the transparency of the material
    fn transparency(&self) -> f64;
}

// Implement the Material trait for a DiffuseMaterial
pub struct DiffuseMaterial {
    diffuse_color: [f64; 3],
    specular_color: [f64; 3],
}

impl DiffuseMaterial {
    // Create a new DiffuseMaterial
    pub fn new(diffuse_color: [f64; 3], specular_color: [f64; 3]) -> Self {
        DiffuseMaterial {
            diffuse_color,
            specular_color,
        }
    }
}

impl Material for DiffuseMaterial {
    // Get the diffuse color of the material
    fn diffuse_color(&self) -> [f64; 3] {
        self.diffuse_color
    }

    // Get the specular color of the material
    fn specular_color(&self) -> [f64; 3] {
        self.specular_color
    }

    // Get the refractive index of the material
    fn refractive_index(&self) -> f64 {
        1.0
    }

    // Get the transparency of the material
    fn transparency(&self) -> f64 {
        1.0
    }
}

// Implement the Material trait for a SpecularMaterial
pub struct SpecularMaterial {
    specular_color: [f64; 3],
    refractive_index: f64,
}

impl SpecularMaterial {
    // Create a new SpecularMaterial
    pub fn new(specular_color: [f64; 3], refractive_index: f64) -> Self {
        SpecularMaterial {
            specular_color,
            refractive_index,
        }
    }
}

impl Material for SpecularMaterial {
    // Get the diffuse color of the material
    fn diffuse_color(&self) -> [f64; 3] {
        [0.0; 3]
    }

    // Get the specular color of the material
    fn specular_color(&self) -> [f64; 3] {
        self.specular_color
    }

    // Get the refractive index of the material
    fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    // Get the transparency of the material
    fn transparency(&self) -> f64 {
        0.0
    }
}

// Implement the Material trait for a TransparentMaterial
pub struct TransparentMaterial {
    diffuse_color: [f64; 3],
    transparency: f64,
}

impl TransparentMaterial {
    // Create a new TransparentMaterial
    pub fn new(diffuse_color: [f64; 3], transparency: f64) -> Self {
        TransparentMaterial {
            diffuse_color,
            transparency,
        }
    }
}

impl Material for TransparentMaterial {
    // Get the diffuse color of the material
    fn diffuse_color(&self) -> [f64; 3] {
        self.diffuse_color
    }

    // Get the specular color of the material
    fn specular_color(&self) -> [f64; 3] {
        [0.0; 3]
    }

    // Get the refractive index of the material
    fn refractive_index(&self) -> f64 {
        1.5
    }

    // Get the transparency of the material
    fn transparency(&self) -> f64 {
        self.transparency
    }
}