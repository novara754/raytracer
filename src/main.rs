use std::path::PathBuf;

use camera::Camera;
use clap::Parser;
use image::{ImageFormat, RgbImage};

use crate::vec3::Vec3;

mod camera;
mod materials;
mod objects;
mod ray;
mod scenes;
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
    #[clap(long)]
    width: Option<u32>,

    /// Height of the output image.
    #[clap(long)]
    height: Option<u32>,

    /// Vertical field of view.
    #[clap(long)]
    fov: Option<f64>,

    /// Distance of the focal point from the camera.
    #[clap(long)]
    focus_distance: Option<f64>,

    /// Determines the strength of the depth of field effect.
    #[clap(long)]
    defocus_angle: Option<f64>,

    /// Number of samples (rays) per pixel.
    #[clap(long)]
    samples: Option<u32>,

    /// Maximum amount of times a ray can get hit and bounce from objects.
    #[clap(long)]
    max_bounces: Option<u32>,

    /// Selects which scenes to render
    #[clap(long, default_value_t = Scene::BouncingSpheres)]
    scene: Scene,
}

fn main() {
    let args = Args::parse();

    let scene: &dyn scenes::scene::Scene = match args.scene {
        Scene::BouncingSpheres => &scenes::bouncing_spheres::BouncingSpheresScene,
        Scene::CheckeredSpheres => &scenes::checkered_spheres::CheckeredSpheresScene,
        Scene::Earth => &scenes::earth::EarthScene,
        Scene::Quads => &scenes::quads::QuadsScene,
        Scene::SimpleLight => &scenes::simple_light::SimpleLightScene,
        Scene::BouncingSpheresWithLight => {
            &scenes::bouncing_spheres_with_light::BouncingSpheresWithLightScene
        }
        Scene::EmptyCornellBox => &scenes::empty_cornell_box::EmptyCornellBoxScene,
        Scene::CornellBox => &scenes::cornell_box::CornellBoxScene,
    };

    let default_settings = scene.default_settings();

    let camera = Camera::new(
        args.width.unwrap_or(default_settings.width),
        args.height.unwrap_or(default_settings.height),
        default_settings.camera_eye,
        default_settings.camera_target,
        Vec3(0.0, 1.0, 0.0),
        args.fov.unwrap_or(default_settings.fov),
        args.focus_distance
            .unwrap_or(default_settings.focus_distance),
        args.defocus_angle.unwrap_or(default_settings.defocus_angle),
        args.samples.unwrap_or(default_settings.samples),
        args.max_bounces.unwrap_or(default_settings.max_bounces),
        default_settings.background_color,
    );

    let world = scene.world();

    let mut img = RgbImage::new(camera.width, camera.height);
    camera.render(&mut img, &world);
    img.save_with_format(args.output_filename, ImageFormat::Png)
        .expect("failed to save output image");
}
