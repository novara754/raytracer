use crate::{
    hittable::HitRecord,
    ray::Ray,
    util::{rand_unit_vec3, reflect},
    vec3::Color,
};

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut dir = rec.normal + rand_unit_vec3();
        if dir.near_zero() {
            dir = rec.normal;
        }
        let ray = Ray::new(rec.position, dir);
        let attenuation = self.albedo;
        Some(ScatterResult { ray, attenuation })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let dir = reflect(ray.direction.normalize(), rec.normal);
        let fuzzed_dir = dir + self.fuzziness * rand_unit_vec3();
        let ray = Ray::new(rec.position, fuzzed_dir);
        let attenuation = self.albedo;
        Some(ScatterResult { ray, attenuation })
    }
}
