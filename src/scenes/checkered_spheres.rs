use std::sync::Arc;

use crate::{
    materials::{material::Lambertian, texture::CheckerTexture},
    objects::{bvh::Bvh, hittable::Hittable, sphere::Sphere},
    vec3::{Color, Vec3},
};

use super::scene::{Scene, Settings};

pub struct CheckeredSpheresScene;

impl Scene for CheckeredSpheresScene {
    fn default_settings(&self) -> Settings {
        Settings {
            camera_eye: Vec3(13.0, 2.0, 3.0),
            camera_target: Vec3(0.0, 0.0, 0.0),
            width: 1280,
            height: 720,
            fov: 20.0,
            focus_distance: 10.0,
            defocus_angle: 0.6,
            samples: 100,
            max_bounces: 50,
            background_color: None,
        }
    }

    fn world(&self) -> Bvh {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];

        let checker_texture = Arc::new(CheckerTexture::from_colors(
            0.32,
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
        ));
        let material = Arc::new(Lambertian::new(checker_texture));

        objects.push(Arc::new(Sphere::stationary(
            Vec3(0.0, -10.0, 0.0),
            10.0,
            material.clone(),
        )));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(0.0, 10.0, 0.0),
            10.0,
            material,
        )));

        Bvh::new(objects.as_slice())
    }
}
