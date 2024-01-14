use image::RgbImage;
use rand::random;

use crate::hittable::Hittable;
use crate::material::ScatterResult;
use crate::ray::Ray;
use crate::util::{linear_to_gamma, Interval};
use crate::vec3::{Color, Vec3};

pub struct Camera {
    center: Vec3,
    width: u32,
    height: u32,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = (width as f64) / (height as f64);

        let viewport_width = 2.0;
        let viewport_height = viewport_width / aspect_ratio;

        let focal_length = 1.0;

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (width as f64);
        let pixel_delta_v = viewport_v / (height as f64);

        let center = Vec3(0.0, 0.0, 0.0);
        let viewport_upper_left =
            center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            center,
            width,
            height,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: 10,
            max_depth: 50,
        }
    }

    pub fn render(&self, img: &mut RgbImage, world: &dyn Hittable) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let pixel_center = self.pixel00_loc
                        + (x as f64 * self.pixel_delta_u)
                        + (y as f64 * self.pixel_delta_v);

                    let px = -0.5 + random::<f64>();
                    let py = -0.5 + random::<f64>();

                    let pixel_sample =
                        pixel_center + (px * self.pixel_delta_u) + (py * self.pixel_delta_v);

                    let ray_direction = pixel_sample - self.center;
                    let ray = Ray::new(self.center, ray_direction);

                    pixel_color += self.ray_color(&ray, 0, world);
                }
                let pixel_color = pixel_color / (self.samples_per_pixel as f64);
                let gamma_corrected = linear_to_gamma(pixel_color);
                img.put_pixel(x, y, gamma_corrected.into());
            }
        }
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth >= self.max_depth {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(ray, Interval(0.001, f64::INFINITY)) {
            if let Some(ScatterResult { ray, attenuation }) = rec.material.scatter(ray, &rec) {
                return attenuation * self.ray_color(&ray, depth + 1, world);
            } else {
                Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
