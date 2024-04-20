use std::sync::Arc;

use crate::{
    materials::{material::Lambertian, texture::ImageTexture},
    objects::{bvh::Bvh, hittable::Hittable, sphere::Sphere, world::World},
    vec3::Vec3,
};

use super::scene::{Scene, Settings};

pub struct EarthScene;

impl Scene for EarthScene {
    fn default_settings(&self) -> Settings {
        Settings {
            camera_eye: Vec3(0.0, 0.0, 12.0),
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

    fn world(&self) -> World {
        let mut world = World::new();

        let mut objects: Vec<Arc<dyn Hittable>> = vec![];

        let earthmap = image::open("./assets/earthmap.jpg").unwrap().into_rgb8();
        let img_texture = Arc::new(ImageTexture::new(earthmap));
        let material = world.register_material(Box::new(Lambertian::new(img_texture)));

        objects.push(Arc::new(Sphere::stationary(
            Vec3(0.0, 0.0, 0.0),
            2.0,
            material,
        )));

        world.set_bvh(Bvh::new(objects.as_slice()));

        world
    }
}
