use std::sync::Arc;

use crate::aabb::Aabb;
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
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn with_face_normal(
        ray: Ray,
        t: f64,
        position: Vec3,
        outward_normal: Vec3,
        material: Arc<dyn Material>,
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

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
    bounding_box: Aabb,
}

impl HittableList {
    #[allow(unused)]
    pub fn from_slice(objects: &[Arc<dyn Hittable>]) -> Self {
        let mut bounding_box = Aabb::empty();
        for object in objects {
            bounding_box = Aabb::combine(bounding_box, object.bounding_box());
        }

        Self {
            objects: objects.to_vec(),
            bounding_box,
        }
    }

    #[allow(unused)]
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bounding_box = Aabb::combine(self.bounding_box, object.bounding_box());
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

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
