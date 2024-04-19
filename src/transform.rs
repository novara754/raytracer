use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    util::Interval,
    vec3::Vec3,
};

pub struct Translate {
    offset: Vec3,
    object: Arc<dyn Hittable>,
}

impl Translate {
    pub fn new(offset: Vec3, object: Arc<dyn Hittable>) -> Self {
        Self { offset, object }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if let Some(mut hit) = self.object.hit(&offset_ray, allowed_t) {
            hit.position += self.offset;
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.object.bounding_box() + self.offset
    }
}

pub struct RotateY {
    angle: f64,
    object: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(angle: f64, object: Arc<dyn Hittable>) -> Self {
        let bbox = object.bounding_box();

        let mut min = Vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.1 + (1.0 - i as f64) * bbox.x.0;
                    let y = j as f64 * bbox.y.1 + (1.0 - j as f64) * bbox.y.0;
                    let z = k as f64 * bbox.z.1 + (1.0 - k as f64) * bbox.z.0;

                    let new_x = angle.cos() * x + angle.sin() * z;
                    let new_z = -angle.sin() * x + angle.sin() * z;

                    let tester = Vec3(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            angle,
            object,
            bbox: Aabb::span_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        let Ray {
            mut origin,
            mut direction,
            ..
        } = *ray;

        origin[0] = self.angle.cos() * origin[0] - self.angle.sin() * origin[2];
        origin[2] = self.angle.sin() * origin[0] + self.angle.cos() * origin[2];

        direction[0] = self.angle.cos() * direction[0] - self.angle.sin() * direction[2];
        direction[2] = self.angle.sin() * direction[0] + self.angle.cos() * direction[2];

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(mut hit) = self.object.hit(&rotated_ray, allowed_t) {
            hit.position[0] =
                self.angle.cos() * hit.position[0] + self.angle.sin() * hit.position[2];
            hit.position[2] =
                -self.angle.sin() * hit.position[0] + self.angle.cos() * hit.position[2];

            hit.normal[0] = self.angle.cos() * hit.normal[0] + self.angle.sin() * hit.normal[2];
            hit.normal[2] = -self.angle.sin() * hit.normal[0] + self.angle.cos() * hit.normal[2];

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
