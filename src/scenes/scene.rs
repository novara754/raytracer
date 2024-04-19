use crate::{
    objects::bvh::Bvh,
    vec3::{Color, Vec3},
};

pub struct Settings {
    /// Position of the camera.
    pub camera_eye: Vec3,

    /// Target position for the camera.
    pub camera_target: Vec3,

    /// Width of the output image.
    pub width: u32,

    /// Height of the output image.
    pub height: u32,

    /// Vertical field of view.
    pub fov: f64,

    /// Distance of the focal point from the camera.
    pub focus_distance: f64,

    /// Determines the strength of the depth of field effect.
    pub defocus_angle: f64,

    /// Number of samples (rays) per pixel.
    pub samples: u32,

    /// Maximum amount of times a ray can get hit and bounce from objects.
    pub max_bounces: u32,

    /// Background color to use for the render, None to use a basic sky-like gradient
    pub background_color: Option<Color>,
}

pub trait Scene {
    fn default_settings(&self) -> Settings;
    fn world(&self) -> Bvh;
}
