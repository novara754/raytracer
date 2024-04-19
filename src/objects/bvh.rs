use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use crate::{
    objects::aabb::Aabb,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    util::Interval,
};

pub struct Bvh {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: Aabb,
}

impl Bvh {
    pub fn new(objects: &[Arc<dyn Hittable>]) -> Self {
        let mut objects = objects.to_vec();

        let axis = rand::thread_rng().gen_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        let left;
        let right;

        if objects.len() == 1 {
            left = Arc::clone(&objects[0]);
            right = Arc::clone(&objects[0]);
        } else if objects.len() == 2 {
            if comparator(objects[0].as_ref(), objects[1].as_ref()).is_lt() {
                left = Arc::clone(&objects[0]);
                right = Arc::clone(&objects[1]);
            } else {
                left = Arc::clone(&objects[1]);
                right = Arc::clone(&objects[0]);
            }
        } else {
            objects.sort_by(|a, b| comparator(a.as_ref(), b.as_ref()));
            let mid = objects.len() / 2;
            left = Arc::new(Bvh::new(&objects[..mid]));
            right = Arc::new(Bvh::new(&objects[mid..]));
        }

        let bounding_box = Aabb::combine(left.bounding_box(), right.bounding_box());

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, allowed_t) {
            return None;
        }

        let hit_left = self.left.hit(ray, allowed_t);

        // If the left bounding volume was hit we still need to
        // check the right one in case it contains an object that is further in front.
        // But we can set its max distance to be where we hit the left volume already.
        let max = if let Some(ref rec) = hit_left {
            rec.t
        } else {
            allowed_t.1
        };
        let hit_right = self.right.hit(ray, Interval(allowed_t.0, max));

        if let Some(rec) = hit_right {
            Some(rec)
        } else {
            hit_left
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis_idx: usize) -> Ordering {
    a.bounding_box()
        .axis(axis_idx)
        .0
        .total_cmp(&b.bounding_box().axis(axis_idx).0)
}

fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
    box_compare(a, b, 2)
}
