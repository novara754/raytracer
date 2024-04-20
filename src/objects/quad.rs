use crate::{
    materials::{material::MaterialRef, texture::TexCoord},
    objects::{
        aabb::Aabb,
        hittable::{HitRecord, Hittable},
    },
    ray::Ray,
    util::Interval,
    vec3::Vec3,
};

pub struct Quad {
    pub starting_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub material: MaterialRef,

    normal: Vec3,
    w: Vec3,
    big_d: f64,

    bounding_box: Aabb,
}

impl Quad {
    pub fn new(starting_corner: Vec3, u: Vec3, v: Vec3, material: MaterialRef) -> Self {
        let box1 = Aabb::span_points(starting_corner, starting_corner + u + v);
        let box2 = Aabb::span_points(starting_corner + u, starting_corner + v);
        let bbox = Aabb::combine(box1, box2);

        let n = u.cross(v);
        let normal = n.normalize();
        let w = n / n.dot(n);
        let big_d = normal.dot(starting_corner);

        Self {
            starting_corner,
            u,
            v,
            material,

            normal,
            w,
            big_d,

            bounding_box: bbox,
        }
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }

    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        // Plane: n * v = D
        // Ray hits plane if: n * R(t) = d
        // Therefore: t = D - n * O / (n * d)

        let denom = self.normal.dot(ray.direction);
        if denom.abs() < f64::EPSILON {
            return None;
        }

        let t = (self.big_d - self.normal.dot(ray.origin)) / denom;
        if !allowed_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let p = intersection - self.starting_corner;
        let alpha = self.w.dot(p.cross(self.v));
        let beta = self.w.dot(self.u.cross(p));

        if !(0.0..=1.0).contains(&alpha) || !(0.0..=1.0).contains(&beta) {
            return None;
        }

        Some(HitRecord::with_face_normal(
            *ray,
            t,
            intersection,
            TexCoord::new(alpha, beta),
            self.normal,
            self.material,
        ))
    }
}
