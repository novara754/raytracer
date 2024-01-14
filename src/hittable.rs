use std::rc::Rc;

use crate::material::Material;
use crate::ray::Ray;
use crate::util::Interval;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub position: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn with_face_normal(
        ray: Ray,
        t: f64,
        position: Vec3,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        Self {
            position,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn from_slice(objects: &[Rc<dyn Hittable>]) -> Self {
        Self {
            objects: objects.to_vec(),
        }
    }

    #[allow(unused)]
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        let mut maybe_rec = None;
        let min = allowed_t.0;
        let mut closest = allowed_t.1;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, Interval(min, closest)) {
                closest = rec.t;
                maybe_rec = Some(rec);
            }
        }

        maybe_rec
    }
}
