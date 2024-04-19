use std::sync::Arc;

use crate::{
    materials::{
        material::{Dialectric, Lambertian, Material, Metal},
        texture::CheckerTexture,
    },
    objects::{bvh::Bvh, hittable::Hittable, sphere::Sphere},
    util::{rand_f64, rand_vec3},
    vec3::{Color, Vec3},
};

use super::scene::{Scene, Settings};

pub struct BouncingSpheresScene;

impl Scene for BouncingSpheresScene {
    fn default_settings(&self) -> Settings {
        Settings {
            camera_eye: Vec3(13.0, 2.0, 3.0),
            camera_target: Vec3(0.0, 0.0, -1.0),
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
        let material_ground = Arc::new(Lambertian::new(checker_texture));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(0.0, -1000.0, 0.0),
            1000.0,
            material_ground,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rand_f64(0.0, 1.0);
                let start_center = Vec3(
                    a as f64 + 0.9 * rand_f64(0.0, 1.0),
                    0.2,
                    b as f64 + 0.9 * rand_f64(0.0, 1.0),
                );

                if (start_center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                    let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                        let albedo = rand_vec3(0.0, 1.0) * rand_vec3(0.0, 1.0);
                        Arc::new(Lambertian::from_color(albedo))
                    } else if choose_mat < 0.95 {
                        let albedo = rand_vec3(0.5, 1.0);
                        let fuzz = rand_f64(0.0, 0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    } else {
                        Arc::new(Dialectric::new(1.5))
                    };

                    let end_center = start_center + Vec3(0.0, rand_f64(0.0, 0.5), 0.0);
                    objects.push(Arc::new(Sphere::moving(start_center, end_center, 0.2, mat)));
                }
            }
        }

        let material1 = Arc::new(Dialectric::new(1.5));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));

        let material2 = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));

        let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        objects.push(Arc::new(Sphere::stationary(
            Vec3(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));

        Bvh::new(objects.as_slice())
    }
}
