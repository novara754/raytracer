use clap::Parser;
use cube::cube;
use image::{ImageFormat, RgbImage};
use material::DiffuseLight;
use quad::Quad;
use std::path::PathBuf;
use std::sync::Arc;
use texture::ImageTexture;

use crate::{
    bvh::Bvh,
    camera::Camera,
    hittable::Hittable,
    material::{Dialectric, Lambertian, Material, Metal},
    sphere::Sphere,
    texture::CheckerTexture,
    util::{rand_f64, rand_vec3},
    vec3::{Color, Vec3},
};

mod aabb;
mod bvh;
mod camera;
mod cube;
mod hittable;
mod material;
mod quad;
mod ray;
mod sphere;
mod texture;
mod util;
mod vec3;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Scene {
    BouncingSpheres,
    CheckeredSpheres,
    Earth,
    Quads,
    SimpleLight,
    BouncingSpheresWithLight,
    EmptyCornellBox,
    CornellBox,
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Scene::BouncingSpheres => write!(f, "bouncing-spheres"),
            Scene::CheckeredSpheres => write!(f, "checkered-spheres"),
            Scene::Earth => write!(f, "earth"),
            Scene::Quads => write!(f, "quads"),
            Scene::SimpleLight => write!(f, "simple-light"),
            Scene::BouncingSpheresWithLight => write!(f, "bouncing-spheres-with-light"),
            Scene::EmptyCornellBox => write!(f, "empty-cornell-box"),
            Scene::CornellBox => write!(f, "cornell-box"),
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the output image.
    #[clap(short, long, default_value = "out.png")]
    output_filename: PathBuf,

    /// Width of the output image.
    #[clap(long, default_value_t = 1280)]
    width: u32,

    /// Height of the output image.
    #[clap(long, default_value_t = 720)]
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

    /// Selects which scenes to render
    #[clap(long, default_value_t = Scene::BouncingSpheres)]
    scene: Scene,
}

fn bouncing_spheres(args: &Args) -> (Camera, Bvh) {
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

    (camera, Bvh::new(objects.as_slice()))
}

fn checkered_spheres(args: &Args) -> (Camera, Bvh) {
    let camera = Camera::new(
        args.width,
        args.height,
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        args.fov,
        args.focus_distance,
        args.defocus_angle,
        args.samples,
        args.max_bounces,
    );

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

    (camera, Bvh::new(objects.as_slice()))
}

fn earth(args: &Args) -> (Camera, Bvh) {
    let camera = Camera::new(
        args.width,
        args.height,
        Vec3(0.0, 0.0, 12.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        args.fov,
        args.focus_distance,
        args.defocus_angle,
        args.samples,
        args.max_bounces,
    );

    let mut objects: Vec<Arc<dyn Hittable>> = vec![];

    let earthmap = image::open("./assets/earthmap.jpg").unwrap().into_rgb8();
    let img_texture = Arc::new(ImageTexture::new(earthmap));
    let material = Arc::new(Lambertian::new(img_texture));

    objects.push(Arc::new(Sphere::stationary(
        Vec3(0.0, 0.0, 0.0),
        2.0,
        material.clone(),
    )));

    (camera, Bvh::new(objects.as_slice()))
}

fn quads(args: &Args) -> (Camera, Bvh) {
    let camera = Camera::new(
        args.width,
        args.height,
        Vec3(0.0, 0.0, 9.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        args.fov,
        args.focus_distance,
        args.defocus_angle,
        args.samples,
        args.max_bounces,
    );

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

    (camera, Bvh::new(objects.as_slice()))
}

fn simple_light(args: &Args) -> (Camera, Bvh) {
    let mut camera = Camera::new(
        args.width,
        args.height,
        Vec3(26.0, 3.0, 6.0),
        Vec3(0.0, 2.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        args.fov,
        args.focus_distance,
        args.defocus_angle,
        args.samples,
        args.max_bounces,
    );
    camera.background_color = Some(Color::new(0.0, 0.0, 0.0));

    let mut objects: Vec<Arc<dyn Hittable>> = vec![];

    let lambert = Arc::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.5)));
    objects.push(Arc::new(Sphere::stationary(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        lambert.clone(),
    )));
    objects.push(Arc::new(Sphere::stationary(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        lambert,
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    objects.push(Arc::new(Quad::new(
        Vec3(3.0, 1.0, -2.0),
        Vec3(2.0, 0.0, 0.0),
        Vec3(0.0, 2.0, 0.0),
        difflight,
    )));

    (camera, Bvh::new(objects.as_slice()))
}

fn bouncing_spheres_with_light(args: &Args) -> (Camera, Bvh) {
    let mut camera = Camera::new(
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
    camera.background_color = Some(Color::new(0.0, 0.0, 0.0));

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

    let diffuse_light = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    objects.push(Arc::new(Quad::new(
        Vec3(-4.0, 5.0, -4.0),
        Vec3(8.0, 0.0, 0.0),
        Vec3(0.0, 0.0, 8.0),
        diffuse_light,
    )));

    (camera, Bvh::new(objects.as_slice()))
}

fn empty_cornell_box(args: &Args) -> (Camera, Bvh) {
    let mut camera = Camera::new(
        args.width,
        args.height,
        Vec3(278.0, 278.0, -800.0),
        Vec3(278.0, 278.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        40.0, // args.fov,
        args.focus_distance,
        0.0, // args.defocus_angle,
        args.samples,
        args.max_bounces,
    );
    camera.background_color = Some(Color::new(0.0, 0.0, 0.0));

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

    (camera, Bvh::new(objects.as_slice()))
}

fn cornell_box(args: &Args) -> (Camera, Bvh) {
    let mut camera = Camera::new(
        args.width,
        args.height,
        Vec3(278.0, 278.0, -800.0),
        Vec3(278.0, 278.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        40.0, // args.fov,
        args.focus_distance,
        0.0, // args.defocus_angle,
        args.samples,
        args.max_bounces,
    );
    camera.background_color = Some(Color::new(0.0, 0.0, 0.0));

    let mut objects: Vec<Arc<dyn Hittable>> = vec![];

    let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(5.0, 5.0, 5.0)));

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
    // objects.push(Arc::new(Quad::new(
    //     Vec3(343.0, 554.0, 332.0),
    //     Vec3(-130.0, 0.0, 0.0),
    //     Vec3(0.0, 0.0, -105.0),
    //     light,
    // )));
    objects.push(Arc::new(Quad::new(
        Vec3(500.0, 554.0, 500.0),
        Vec3(-450.0, 0.0, 0.0),
        Vec3(0.0, 0.0, -450.0),
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
        white.clone(),
    )));

    objects.push(Arc::new(cube(
        Vec3(130.0, 0.0, 65.0),
        Vec3(295.0, 165.0, 230.0),
        white.clone(),
    )));
    objects.push(Arc::new(cube(
        Vec3(265.0, 0.0, 295.0),
        Vec3(430.0, 330.0, 460.0),
        white.clone(),
    )));

    (camera, Bvh::new(objects.as_slice()))
}

fn main() {
    let args = Args::parse();

    let (camera, world) = match args.scene {
        Scene::BouncingSpheres => bouncing_spheres(&args),
        Scene::CheckeredSpheres => checkered_spheres(&args),
        Scene::Earth => earth(&args),
        Scene::Quads => quads(&args),
        Scene::SimpleLight => simple_light(&args),
        Scene::BouncingSpheresWithLight => bouncing_spheres_with_light(&args),
        Scene::EmptyCornellBox => empty_cornell_box(&args),
        Scene::CornellBox => cornell_box(&args),
    };

    let mut img = RgbImage::new(args.width, args.height);
    camera.render(&mut img, &world);
    img.save_with_format(args.output_filename, ImageFormat::Png)
        .expect("failed to save output image");
}
