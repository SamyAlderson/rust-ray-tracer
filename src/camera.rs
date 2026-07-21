// Camera implementation
// This was tricky, but we're using a simple pinhole camera model for this project
// See https://en.wikipedia.org/wiki/Pinhole_camera_model for more details

use nalgebra::{Point3, Vector3, Rodrigues};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Point3<f64>,
    pub look_at: Point3<f64>,
    pub up: Vector3<f64>,
    pub fov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_distance: f64,
}

impl Camera {
    // Create a new camera instance
    pub fn new(
        origin: Point3<f64>,
        look_at: Point3<f64>,
        up: Vector3<f64>,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        Self {
            origin,
            look_at,
            up,
            fov,
            aspect_ratio,
            aperture,
            focus_distance,
        }
    }

    // Calculate the camera's view matrix
    // This is a simplified version, see https://en.wikipedia.org/wiki/View-matrix for more details
    pub fn view_matrix(&self) -> nalgebra::Matrix4<f64> {
        let direction = (self.look_at - self.origin).normalize();
        let right = direction.cross(&self.up).normalize();
        let up = right.cross(&direction).normalize();

        let rotation = Rodrigues::new(right, up, direction).into_matrix();

        rotation * nalgebra::Matrix4::new(
            0.0,
            -1.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
        )
    }

    // Calculate the camera's projection matrix
    // We're using a simplified version of the perspective projection matrix
    // See https://en.wikipedia.org/wiki/Perspective_projection for more details
    pub fn projection_matrix(&self) -> nalgebra::Matrix4<f64> {
        let f = 1.0 / (self.fov * (self.aspect_ratio as f64));
        let n = 1.0 / (self.focus_distance - self.aperture / 2.0);
        let f_n = -1.0;

        nalgebra::Matrix4::new(
            f * n,
            0.0,
            0.0,
            0.0,
            0.0,
            f_n * n,
            0.0,
            0.0,
            0.0,
            0.0,
            n,
            -1.0,
            0.0,
            0.0,
            0.0,
            (self.focus_distance + self.aperture / 2.0) * n,
        )
    }
}

// Example usage:
fn main() {
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -1.0),
        Vector3::z_axis(),
        60.0,
        16.0 / 9.0,
        0.01,
        1.0,
    );

    println!("{:?}", camera.view_matrix());
    println!("{:?}", camera.projection_matrix());
}