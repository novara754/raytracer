use std::path::PathBuf;
use std::sync::Arc;

use bvh::Bvh;
use clap::Parser;
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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the output image.
    #[clap(short, long, default_value = "out.png")]
    output_filename: PathBuf,

    /// Width of the output image.
    #[clap(short, long, default_value_t = 1280)]
    width: u32,

    /// Height of the output image.
    #[clap(short, long, default_value_t = 720)]
    height: u32,

    /// Vertical field of view.
    #[clap(long, default_value_t = 20.0)]
    fov: f64,

    /// Distance of the focal point from the camera.
    #[clap(long, default_value_t = 10.0)]
    focus_distance: f64,

    /// Determines the strength of the depth of field effect.
    #[clap(long, default_value_t = 0.6)]
    defocus_angle: f64,

    /// Number of samples (rays) per pixel.
    #[clap(long, default_value_t = 100)]
    samples: u32,

    /// Maximum amount of times a ray can get hit and bounce from objects.
    #[clap(long, default_value_t = 50)]
    max_bounces: u32,
}

fn main() {
    let args = Args::parse();

    let camera = Camera::new(
        args.width,
        args.height,
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        args.fov,
        args.focus_distance,
        args.defocus_angle,
        args.samples,
        args.max_bounces,
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
                let mat: Arc<dyn Material> = if choose_mat < 0.8 {
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

    let mut img = RgbImage::new(args.width, args.height);
    camera.render(&mut img, &world);
    img.save_with_format(args.output_filename, ImageFormat::Png)
        .expect("failed to save output image");
}
