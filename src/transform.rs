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
    bounding_box: Aabb,
}

impl Translate {
    pub fn new(offset: Vec3, object: Arc<dyn Hittable>) -> Self {
        let bounding_box = object.bounding_box() + offset;

        Self {
            offset,
            object,
            bounding_box,
        }
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
        self.bounding_box
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    bounding_box: Aabb,

    cos_angle: f64,
    sin_angle: f64,
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
            object,
            bounding_box: Aabb::span_points(min, max),

            cos_angle: angle.cos(),
            sin_angle: angle.sin(),
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

        origin[0] = self.cos_angle * origin[0] - self.sin_angle * origin[2];
        origin[2] = self.sin_angle * origin[0] + self.cos_angle * origin[2];

        direction[0] = self.cos_angle * direction[0] - self.sin_angle * direction[2];
        direction[2] = self.sin_angle * direction[0] + self.cos_angle * direction[2];

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(mut hit) = self.object.hit(&rotated_ray, allowed_t) {
            hit.position[0] = self.cos_angle * hit.position[0] + self.sin_angle * hit.position[2];
            hit.position[2] = -self.sin_angle * hit.position[0] + self.cos_angle * hit.position[2];

            hit.normal[0] = self.cos_angle * hit.normal[0] + self.sin_angle * hit.normal[2];
            hit.normal[2] = -self.sin_angle * hit.normal[0] + self.cos_angle * hit.normal[2];

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
