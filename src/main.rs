use std::rc::Rc;

use image::{ImageFormat, RgbImage};
use material::{Dialectric, Lambertian, Metal};
use vec3::Color;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

fn main() {
    let image_width = 640;
    let image_height = 480;

    let camera = Camera::new(
        image_width,
        image_height,
        Vec3(-2.0, 2.0, 1.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
    );

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dialectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let world = HittableList::from_slice(&[
        Rc::new(Sphere::new(
            Vec3(0.0, -100.5, -1.0),
            100.0,
            material_ground.clone(),
        )),
        Rc::new(Sphere::new(
            Vec3(0.0, 0.0, -1.0),
            0.5,
            material_center.clone(),
        )),
        Rc::new(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        )),
        Rc::new(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            -0.4,
            material_left.clone(),
        )),
        Rc::new(Sphere::new(
            Vec3(1.0, 0.0, -1.0),
            0.5,
            material_right.clone(),
        )),
    ]);

    let mut img = RgbImage::new(image_width, image_height);
    camera.render(&mut img, &world);
    img.save_with_format("./out.png", ImageFormat::Png)
        .expect("failed to save output image");
}
