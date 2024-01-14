use std::rc::Rc;

use image::{ImageFormat, RgbImage};

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let image_width = 640;
    let image_height = 480;

    let camera = Camera::new(image_width, image_height);

    let sphere1 = Rc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Rc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0));
    let world = HittableList::from_slice(&[sphere1, sphere2]);

    let mut img = RgbImage::new(image_width, image_height);
    camera.render(&mut img, &world);
    img.save_with_format("./out.png", ImageFormat::Png)
        .expect("failed to save output image");
}
