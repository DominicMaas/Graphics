use cgmath::{Matrix4, Rad};

use crate::OPENGL_TO_WGPU_MATRIX;

pub trait Projection {
    /// When the window resizes, this updates the internal
    /// aspect ratio to ensure everything still looks correct
    fn resize(&mut self, width: u32, height: u32);

    /// The current window size, this is used for internal calculations
    fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32>;

    /// Calculate the projection matrix for the window
    fn calc_matrix(&self) -> Matrix4<f32>;

    /// Get the near plane for this camera
    fn get_near_plane(&self) -> f32;

    /// Get the far plane for this camera
    fn get_far_plane(&self) -> f32;
}

pub struct PerspectiveProjection {
    pub aspect: f32,
    pub fov: Rad<f32>,
    pub z_near: f32,
    pub z_far: f32,

    window_width: u32,
    window_height: u32,
}

impl PerspectiveProjection {
    pub fn new(width: u32, height: u32, fov: Rad<f32>, z_near: f32, z_far: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fov,
            z_near,
            z_far,

            window_width: width,
            window_height: height,
        }
    }
}

impl Projection for PerspectiveProjection {
    fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * cgmath::perspective(self.fov, self.aspect, self.z_near, self.z_far)
    }

    fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32> {
        winit::dpi::PhysicalSize::new(self.window_width, self.window_height)
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;

        self.aspect = width as f32 / height as f32;
    }

    fn get_near_plane(&self) -> f32 {
        self.z_near
    }

    fn get_far_plane(&self) -> f32 {
        self.z_far
    }
}

pub struct OrthographicProjection {
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,

    window_width: u32,
    window_height: u32,
}

impl OrthographicProjection {
    pub fn new(width: u32, height: u32, z_near: f32, z_far: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            z_near,
            z_far,

            window_width: width,
            window_height: height,
        }
    }
}

impl Projection for OrthographicProjection {
    fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX
            * cgmath::ortho(
                -self.aspect,
                self.aspect,
                -1.0,
                1.0,
                self.z_near,
                self.z_far,
            )
    }

    fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32> {
        winit::dpi::PhysicalSize::new(self.window_width, self.window_height)
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;

        self.aspect = width as f32 / height as f32;
    }

    fn get_near_plane(&self) -> f32 {
        self.z_near
    }

    fn get_far_plane(&self) -> f32 {
        self.z_far
    }
}
