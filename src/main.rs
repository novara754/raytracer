use std::sync::Arc;

use bvh::Bvh;
use hittable::Hittable;
use image::{ImageFormat, RgbImage};
use material::{Dialectric, Lambertian, Material, Metal};
use util::{rand_f64, rand_vec3};
use vec3::Color;

use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod aabb;
mod bvh;
mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

fn main() {
    let image_width = 1920;
    let image_height = 1080;

    let camera = Camera::new(
        image_width,
        image_height,
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.6,
        200,
        50,
    );

    let mut objects: Vec<Arc<dyn Hittable>> = vec![];

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.push(Arc::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64(0.0, 1.0);
            let center = Vec3(
                a as f64 + 0.9 * rand_f64(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rand_f64(0.0, 1.0),
            );

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: Arc<dyn Material + Sync + Send> = if choose_mat < 0.8 {
                    let albedo = rand_vec3(0.0, 1.0) * rand_vec3(0.0, 1.0);
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = rand_vec3(0.5, 1.0);
                    let fuzz = rand_f64(0.0, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dialectric::new(1.5))
                };

                objects.push(Arc::new(Sphere::new(center, 0.2, mat)));
            }
        }
    }

    let material1 = Arc::new(Dialectric::new(1.5));
    objects.push(Arc::new(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    objects.push(Arc::new(Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    objects.push(Arc::new(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let world = Bvh::new(objects.as_slice());

    let mut img = RgbImage::new(image_width, image_height);
    camera.render(&mut img, &world);
    img.save_with_format("./out.png", ImageFormat::Png)
        .expect("failed to save output image");
}
