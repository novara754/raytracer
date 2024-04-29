use std::{f64::INFINITY, sync::Arc};

use crate::{
    materials::{material::MaterialRef, texture::TexCoord},
    ray::Ray,
    util::{rand_f64, Interval},
    vec3::Vec3,
};

use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

pub struct ConstantVolume {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: MaterialRef,
}

impl ConstantVolume {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, phase_function: MaterialRef) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }
}

impl Hittable for ConstantVolume {
    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        // This approach of checking for a hit with the boundary is important for
        // cases where the ray originates inside the volume.
        // This approach also only works for convex shapes.

        // First check if the ray could ever possibly intersect this volume by
        // testing for a hit with the boundary.
        // If there was a hit, that will be the entrance point.
        let mut rec1 = self.boundary.hit(ray, Interval::UNIVERSE)?;
        // Now that we know the entrance point check for another hit as the exit point.
        let mut rec2 = self
            .boundary
            .hit(ray, Interval(rec1.t + 0.0001, INFINITY))?;

        // Previously we checked for hits anywhere, but now we need to constrain
        // it to the allowed interval for t.
        rec1.t = f64::max(rec1.t, allowed_t.0);
        rec2.t = f64::min(rec2.t, allowed_t.1);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = f64::max(rec1.t, 0.0);

        // Next we generate a random distance that the ray must travel through
        // the volume to actually register a hit.
        let hit_distance = self.neg_inv_density * f64::log10(rand_f64(0.0, 1.0));

        // Then we calculate how far the ray actually traversed through the volume
        // using the entrance and exit points.
        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;

        // If the ray didn't actually travel far enough don't register a hit with
        // this volume.
        if hit_distance > distance_inside_boundary {
            return None;
        }

        // Otherwise calculate the appropriate t based on the distance travelled.
        let t = rec1.t + hit_distance / ray_length;

        // The normal vector, uv and front_face values are arbitrary.
        Some(HitRecord {
            position: ray.at(t),
            normal: Vec3(1.0, 0.0, 0.0),
            t,
            uv: TexCoord::new(0.0, 0.0),
            front_face: true,
            material: self.phase_function,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
