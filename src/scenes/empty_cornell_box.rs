use std::sync::Arc;

use crate::{
    materials::material::{DiffuseLight, Lambertian},
    objects::{bvh::Bvh, hittable::Hittable, quad::Quad},
    vec3::{Color, Vec3},
};

use super::scene::{Scene, Settings};

pub struct EmptyCornellBoxScene;

impl Scene for EmptyCornellBoxScene {
    fn default_settings(&self) -> Settings {
        Settings {
            camera_eye: Vec3(278.0, 278.0, -800.0),
            camera_target: Vec3(278.0, 278.0, 0.0),
            width: 720,
            height: 720,
            fov: 38.0,
            focus_distance: 10.0,
            defocus_angle: 0.0,
            samples: 500,
            max_bounces: 50,
            background_color: Some(Color::new(0.0, 0.0, 0.0)),
        }
    }

    fn world(&self) -> Bvh {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];

        let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

        objects.push(Arc::new(Quad::new(
            Vec3(555.0, 0.0, 0.0),
            Vec3(0.0, 555.0, 0.0),
            Vec3(0.0, 0.0, 555.0),
            green,
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(0.0, 0.0, 0.0),
            Vec3(0.0, 555.0, 0.0),
            Vec3(0.0, 0.0, 555.0),
            red,
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(343.0, 554.0, 332.0),
            Vec3(-130.0, 0.0, 0.0),
            Vec3(0.0, 0.0, -105.0),
            light,
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(0.0, 0.0, 0.0),
            Vec3(555.0, 0.0, 0.0),
            Vec3(0.0, 0.0, 555.0),
            white.clone(),
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(555.0, 555.0, 555.0),
            Vec3(-555.0, 0.0, 0.0),
            Vec3(0.0, 0.0, -555.0),
            white.clone(),
        )));
        objects.push(Arc::new(Quad::new(
            Vec3(0.0, 0.0, 555.0),
            Vec3(555.0, 0.0, 0.0),
            Vec3(0.0, 555.0, 0.0),
            white,
        )));

        Bvh::new(objects.as_slice())
    }
}
