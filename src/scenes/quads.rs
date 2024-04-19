use std::sync::Arc;

use crate::{
    materials::material::Lambertian,
    objects::{bvh::Bvh, hittable::Hittable, quad::Quad},
    vec3::{Color, Vec3},
};

use super::scene::{Scene, Settings};

pub struct QuadsScene;

impl Scene for QuadsScene {
    fn default_settings(&self) -> Settings {
        Settings {
            camera_eye: Vec3(0.0, 0.0, 9.0),
            camera_target: Vec3(0.0, 0.0, 0.0),
            width: 720,
            height: 720,
            fov: 80.0,
            focus_distance: 10.0,
            defocus_angle: 0.0,
            samples: 100,
            max_bounces: 50,
            background_color: None,
        }
    }

    fn world(&self) -> Bvh {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];

        let red = Arc::new(Lambertian::from_color(Color::new(1.0, 0.2, 0.2)));
        let green = Arc::new(Lambertian::from_color(Color::new(0.2, 1.0, 0.2)));
        let blue = Arc::new(Lambertian::from_color(Color::new(0.2, 0.2, 1.0)));
        let orange = Arc::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.0)));
        let teal = Arc::new(Lambertian::from_color(Color::new(0.2, 0.8, 0.8)));

        objects.push(Arc::new(Quad::new(
            Vec3(-3.0, -2.0, 5.0),
            Vec3(0.0, 0.0, -4.0),
            Vec3(0.0, 4.0, 0.0),
            red,
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(-2.0, -2.0, 0.0),
            Vec3(4.0, 0.0, 0.0),
            Vec3(0.0, 4.0, 0.0),
            green,
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(3.0, -2.0, 1.0),
            Vec3(0.0, 0.0, 4.0),
            Vec3(0.0, 4.0, 0.0),
            blue,
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(-2.0, 3.0, 1.0),
            Vec3(4.0, 0.0, 0.0),
            Vec3(0.0, 0.0, 4.0),
            orange,
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(-2.0, -3.0, 5.0),
            Vec3(4.0, 0.0, 0.0),
            Vec3(0.0, 0.0, -4.0),
            teal,
        )));

        Bvh::new(objects.as_slice())
    }
}
