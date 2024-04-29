use std::sync::Arc;

use crate::{
    materials::{
        material::{Dialectric, DiffuseLight, Isotropic, Lambertian, Metal},
        texture::ImageTexture,
    },
    objects::{
        bvh::Bvh,
        constant_volume::ConstantVolume,
        cube::cube,
        hittable::Hittable,
        quad::Quad,
        sphere::Sphere,
        transform::{RotateY, Translate},
        world::World,
    },
    util::{rand_f64, rand_vec3},
    vec3::{Color, Vec3},
};

use super::scene::{Scene, Settings};

pub struct EverythingScene;

impl Scene for EverythingScene {
    fn default_settings(&self) -> Settings {
        Settings {
            camera_eye: Vec3(478.0, 278.0, -600.0),
            camera_target: Vec3(278.0, 278.0, 0.0),
            width: 720,
            height: 720,
            fov: 40.0,
            focus_distance: 10.0,
            defocus_angle: 0.0,
            samples: 500,
            max_bounces: 50,
            background_color: Some(Color::new(0.0, 0.0, 0.0)),
        }
    }

    fn world(&self) -> World {
        let mut world = World::new();
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];

        let ground = world.register_material(Box::new(Lambertian::from_color(Color::new(
            0.48, 0.83, 0.53,
        ))));

        let mut boxes: Vec<Arc<dyn Hittable>> = vec![];
        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;

                let x0 = -1000.0 + (i as f64) * w;
                let z0 = -1000.0 + (j as f64) * w;
                let y0 = 0.0;

                let x1 = x0 + w;
                let y1 = rand_f64(1.0, 101.0);
                let z1 = z0 + w;

                boxes.push(Arc::new(cube(Vec3(x0, y0, z0), Vec3(x1, y1, z1), ground)));
            }
        }
        objects.push(Arc::new(Bvh::new(&boxes)));

        let light = world.register_material(Box::new(DiffuseLight::from_color(Color::new(
            7.0, 7.0, 7.0,
        ))));
        objects.push(Arc::new(Quad::new(
            Vec3(123.0, 554.0, 147.0),
            Vec3(300.0, 0.0, 0.0),
            Vec3(0.0, 0.0, 265.0),
            light,
        )));

        let center1 = Vec3(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3(30.0, 0.0, 0.0);
        let sphere_material =
            world.register_material(Box::new(Lambertian::from_color(Color::new(0.7, 0.3, 0.1))));
        objects.push(Arc::new(Sphere::moving(
            center1,
            center2,
            50.0,
            sphere_material,
        )));

        objects.push(Arc::new(Sphere::stationary(
            Vec3(260.0, 150.0, 45.0),
            50.0,
            world.register_material(Box::new(Dialectric::new(1.5))),
        )));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(0.0, 150.0, 145.0),
            50.0,
            world.register_material(Box::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0))),
        )));

        let boundary = Arc::new(Sphere::stationary(
            Vec3(360.0, 150.0, 145.0),
            70.0,
            world.register_material(Box::new(Dialectric::new(1.5))),
        ));
        objects.push(boundary.clone());
        objects.push(Arc::new(ConstantVolume::new(
            boundary,
            0.2,
            world.register_material(Box::new(Isotropic::from_color(Color::new(0.2, 0.4, 0.9)))),
        )));
        let boundary = Arc::new(Sphere::stationary(
            Vec3(0.0, 0.0, 0.0),
            5000.0,
            world.register_material(Box::new(Dialectric::new(1.5))),
        ));
        objects.push(Arc::new(ConstantVolume::new(
            boundary,
            0.0001,
            world.register_material(Box::new(Isotropic::from_color(Color::new(1.0, 1.0, 1.0)))),
        )));

        let earth_image = image::open("./assets/earthmap.jpg").unwrap().into_rgb8();
        let earth_material = world.register_material(Box::new(Lambertian::new(Arc::new(
            ImageTexture::new(earth_image),
        ))));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(400.0, 200.0, 400.0),
            100.0,
            earth_material,
        )));

        let mut boxes2: Vec<Arc<dyn Hittable>> = vec![];
        let white = world.register_material(Box::new(Lambertian::from_color(Color::new(
            0.73, 0.73, 0.73,
        ))));
        for _ in 0..1000 {
            boxes2.push(Arc::new(Sphere::stationary(
                rand_vec3(0.0, 165.0),
                10.0,
                white,
            )));
        }

        objects.push(Arc::new(Translate::new(
            Vec3(-100.0, 270.0, 395.0),
            Arc::new(RotateY::new(15.0, Arc::new(Bvh::new(&boxes2)))),
        )));

        world.set_bvh(Bvh::new(&objects));
        world
    }
}
