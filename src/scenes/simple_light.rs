use std::sync::Arc;

use crate::{
    materials::material::{DiffuseLight, Lambertian},
    objects::{bvh::Bvh, hittable::Hittable, quad::Quad, sphere::Sphere, world::World},
    vec3::{Color, Vec3},
};

use super::scene::{Scene, Settings};

pub struct SimpleLightScene;

impl Scene for SimpleLightScene {
    fn default_settings(&self) -> Settings {
        Settings {
            camera_eye: Vec3(26.0, 3.0, 6.0),
            camera_target: Vec3(0.0, 2.0, 0.0),
            width: 1280,
            height: 720,
            fov: 20.0,
            focus_distance: 10.0,
            defocus_angle: 0.6,
            samples: 1000,
            max_bounces: 10,
            background_color: Some(Color::new(0.0, 0.0, 0.0)),
        }
    }

    fn world(&self) -> World {
        let mut world = World::new();

        let mut objects: Vec<Arc<dyn Hittable>> = vec![];

        let lambert =
            world.register_material(Box::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.5))));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(0.0, -1000.0, 0.0),
            1000.0,
            lambert,
        )));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(0.0, 2.0, 0.0),
            2.0,
            lambert,
        )));

        let difflight = world.register_material(Box::new(DiffuseLight::from_color(Color::new(
            4.0, 4.0, 4.0,
        ))));
        objects.push(Arc::new(Quad::new(
            Vec3(3.0, 1.0, -2.0),
            Vec3(2.0, 0.0, 0.0),
            Vec3(0.0, 2.0, 0.0),
            difflight,
        )));

        world.set_bvh(Bvh::new(objects.as_slice()));

        world
    }
}
