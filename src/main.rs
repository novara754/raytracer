use std::path::PathBuf;

use camera::Camera;
use clap::Parser;
use image::ImageFormat;
use sdl2::{event::Event, pixels::PixelFormatEnum};
use util::linear_to_gamma;
use vec3::Color;

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
    CornellSmoke,
    Everything,
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
            Scene::CornellSmoke => write!(f, "cornell-smoke"),
            Scene::Everything => write!(f, "everything"),
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = false)]
    live_window: bool,

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
        Scene::CornellSmoke => &scenes::cornell_smoke::CornellSmokeScene,
        Scene::Everything => &scenes::everything::EverythingScene,
    };

    let default_settings = scene.default_settings();

    let width = args.width.unwrap_or(default_settings.width);
    let height = args.height.unwrap_or(default_settings.height);

    let camera = Camera::new(
        width,
        height,
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

    if args.live_window {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("raytracer", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut render_target = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
            .unwrap();

        let (tx, rx) = std::sync::mpsc::channel::<(u32, Vec<u8>)>();
        std::thread::spawn(move || {
            let samples_per_iteration = 10;
            let mut num_total_samples = 0;

            let mut linear_color_data =
                vec![Color::new(0.0, 0.0, 0.0); width as usize * height as usize];
            let mut srgb_color_data = vec![0u8; linear_color_data.len() * 3];

            loop {
                camera.render_x_samples(
                    &mut linear_color_data,
                    &world,
                    samples_per_iteration,
                    num_total_samples,
                );

                num_total_samples += samples_per_iteration;

                for (srgb, linear) in srgb_color_data.chunks_mut(3).zip(linear_color_data.iter()) {
                    let non_linear = linear_to_gamma(*linear);
                    srgb[0] = (non_linear.0 * 255.0) as u8;
                    srgb[1] = (non_linear.1 * 255.0) as u8;
                    srgb[2] = (non_linear.2 * 255.0) as u8;
                }

                tx.send((num_total_samples, srgb_color_data.clone()))
                    .unwrap();
            }
        });

        let mut event_loop = sdl_context.event_pump().unwrap();
        'main_loop: loop {
            for event in event_loop.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'main_loop;
                }
            }

            if let Ok((num_total_samples, srgb_color_data)) = rx.try_recv() {
                render_target
                    .update(None, &srgb_color_data, 3 * width as usize)
                    .unwrap();

                canvas
                    .window_mut()
                    .set_title(&format!(
                        "Scene: {} -- Total Number of Samples per Pixel: {}",
                        args.scene, num_total_samples
                    ))
                    .unwrap();
            }

            canvas.copy(&render_target, None, None).unwrap();

            canvas.present();
        }
    } else {
        let img = camera.render(&world);
        img.save_with_format(args.output_filename, ImageFormat::Png)
            .expect("failed to save output image");
    }
}
