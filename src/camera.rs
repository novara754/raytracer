use std::time::Instant;

use image::RgbImage;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rand::random;
use rayon::prelude::*;

use crate::materials::material::ScatterResult;
use crate::objects::hittable::Hittable;
use crate::objects::world::World;
use crate::ray::Ray;
use crate::util::{deg2rad, linear_to_gamma, rand_unit_disc_vec3, Interval};
use crate::vec3::{Color, Vec3};

pub struct Camera {
    eye: Vec3,
    pub width: u32,
    pub height: u32,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f64,
    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3,
    samples_per_pixel: u32,
    max_bounces: u32,
    background_color: Option<Color>,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        width: u32,
        height: u32,
        eye: Vec3,
        look_at: Vec3,
        up: Vec3,
        v_fov: f64,
        focus_dist: f64,
        defocus_angle: f64,
        samples_per_pixel: u32,
        max_bounces: u32,
        background_color: Option<Color>,
    ) -> Self {
        let aspect_ratio = (width as f64) / (height as f64);

        let w = (eye - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let theta = deg2rad(v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / (width as f64);
        let pixel_delta_v = viewport_v / (height as f64);

        let viewport_upper_left = eye - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * deg2rad(defocus_angle / 2.0).tan();
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        Self {
            eye,
            width,
            height,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disc_u,
            defocus_disc_v,
            samples_per_pixel,
            max_bounces,
            background_color,
        }
    }

    pub fn render_x_samples(
        &self,
        data: &mut [Color],
        world: &World,
        num_samples: u32,
        num_previous_samples: u32,
    ) {
        data.par_chunks_mut(self.width as usize)
            .enumerate()
            .for_each(|(row, pixel_row)| {
                pixel_row
                    .par_iter_mut()
                    .enumerate()
                    .for_each(move |(col, pixel)| {
                        *pixel *= num_previous_samples as f64;
                        for _ in 0..num_samples {
                            let ray = self.get_ray(col as u32, row as u32);
                            *pixel += self.ray_color(&ray, 0, world);
                        }
                        *pixel /= num_previous_samples as f64 + num_samples as f64;
                    })
            });
    }

    pub fn render(&self, world: &World) -> RgbImage {
        let start = Instant::now();

        let progress_bar_style =
            ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.}] {pos}/{len} ({eta})")
                .unwrap();

        let mut img = RgbImage::new(self.width, self.height);

        img.par_enumerate_pixels_mut()
            .progress_with_style(progress_bar_style)
            .for_each(|(col, row, pixel)| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(col, row);
                    pixel_color += self.ray_color(&ray, 0, world);
                }
                pixel_color /= self.samples_per_pixel as f64;
                pixel_color = linear_to_gamma(pixel_color);

                *pixel = pixel_color.into();
            });

        let end = Instant::now();

        eprintln!("Time elapsed (seconds): {}", (end - start).as_secs_f64());

        img
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &World) -> Color {
        if depth >= self.max_bounces {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(ray, Interval(0.001, f64::INFINITY)) {
            let material = &world.materials[rec.material.0];
            let emissive_color = material.emit(rec.uv, rec.position);

            if let Some(ScatterResult {
                ray: scattered,
                attenuation,
            }) = material.scatter(ray, &rec)
            {
                let scatter_color = attenuation * self.ray_color(&scattered, depth + 1, world);

                scatter_color + emissive_color
            } else {
                emissive_color
            }
        } else {
            self.background_color.unwrap_or_else(|| {
                let unit_direction = ray.direction.normalize();
                let a = 0.5 * (unit_direction.y() + 1.0);

                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            })
        }
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);

        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();

        let pixel_sample = pixel_center + (px * self.pixel_delta_u) + (py * self.pixel_delta_v);

        let ray_origin = self.defocus_disc_sample();
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random::<f64>();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disc_sample(&self) -> Vec3 {
        if self.defocus_angle <= 0.0 {
            self.eye
        } else {
            let off = rand_unit_disc_vec3();
            self.eye + (off.x() * self.defocus_disc_u) + (off.y() * self.defocus_disc_v)
        }
    }
}
