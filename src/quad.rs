use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    texture::TexCoord,
    util::Interval,
    vec3::Vec3,
};

pub struct Quad {
    pub starting_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub material: Arc<dyn Material>,
}

impl Quad {
    pub fn new(starting_corner: Vec3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        Self {
            starting_corner,
            u,
            v,
            material,
        }
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> Aabb {
        let box1 = Aabb::span_points(self.starting_corner, self.starting_corner + self.u + self.v);
        let box2 = Aabb::span_points(self.starting_corner + self.u, self.starting_corner + self.v);
        Aabb::combine(box1, box2)
    }

    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        // Plane: n * v = D
        // Ray hits plane if: n * R(t) = d
        // Therefore: t = D - n * O / (n * d)

        let n = self.u.cross(self.v);
        let normal = n.normalize();
        let big_d = normal.dot(self.starting_corner);

        let denom = normal.dot(ray.direction);
        if denom.abs() < f64::EPSILON {
            return None;
        }

        let t = (big_d - normal.dot(ray.origin)) / denom;
        if !allowed_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);

        let w = n / n.dot(n);
        let p = intersection - self.starting_corner;

        let alpha = w.dot(p.cross(self.v));
        let beta = w.dot(self.u.cross(p));

        if !(0.0..=1.0).contains(&alpha) || !(0.0..=1.0).contains(&beta) {
            return None;
        }

        Some(HitRecord::with_face_normal(
            *ray,
            t,
            intersection,
            TexCoord::new(alpha, beta),
            normal,
            self.material.clone(),
        ))
    }
}
