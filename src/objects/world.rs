use crate::materials::material::{Material, MaterialRef};

use super::{bvh::Bvh, hittable::Hittable};

pub struct World {
    pub materials: Vec<Box<dyn Material>>,
    pub bvh: Option<Bvh>,
}

impl World {
    pub fn new() -> Self {
        Self {
            materials: vec![],
            bvh: None,
        }
    }

    pub fn register_material(&mut self, material: Box<dyn Material>) -> MaterialRef {
        let new_ref = MaterialRef(self.materials.len());
        self.materials.push(material);
        new_ref
    }

    pub fn set_bvh(&mut self, bvh: Bvh) {
        self.bvh = Some(bvh);
    }

    pub fn bvh(&self) -> &Bvh {
        self.bvh.as_ref().unwrap()
    }
}

impl Hittable for World {
    fn bounding_box(&self) -> super::aabb::Aabb {
        self.bvh().bounding_box()
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        allowed_t: crate::util::Interval,
    ) -> Option<super::hittable::HitRecord> {
        self.bvh().hit(ray, allowed_t)
    }
}
