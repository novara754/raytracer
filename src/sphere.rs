use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::util::Interval;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Sphere {
    pub start_center: Vec3,
    pub move_dir: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
    bounding_box: Aabb,
}

impl Sphere {
    pub fn stationary(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        let r_vec = Vec3(radius, radius, radius);
        let bounding_box = Aabb::span_points(center - r_vec, center + r_vec);

        Self {
            start_center: center,
            move_dir: Vec3(0.0, 0.0, 0.0),
            radius,
            material,
            bounding_box,
        }
    }

    pub fn moving(
        start_center: Vec3,
        end_center: Vec3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let r_vec = Vec3(radius, radius, radius);

        let box1 = Aabb::span_points(start_center - r_vec, start_center + r_vec);
        let box2 = Aabb::span_points(end_center - r_vec, end_center + r_vec);
        let bounding_box = Aabb::combine(box1, box2);

        Self {
            start_center,
            move_dir: end_center - start_center,
            radius,
            material,
            bounding_box,
        }
    }

    pub fn get_center(&self, time: f64) -> Vec3 {
        self.start_center + self.move_dir * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        let center = self.get_center(ray.time);

        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if !allowed_t.surrounds(root) {
            root = (-half_b + sqrt_d) / a;
            if !allowed_t.surrounds(root) {
                return None;
            }
        }

        let position = ray.at(root);
        let outward_normal = (position - center) / self.radius;
        Some(HitRecord::with_face_normal(
            *ray,
            root,
            position,
            outward_normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
