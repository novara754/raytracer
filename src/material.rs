use crate::{
    hittable::HitRecord,
    ray::Ray,
    util::{rand_f64, rand_unit_vec3, reflect, reflectance, refract},
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

pub struct Dialectric {
    refraction_index: f64,
}

impl Dialectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = ray.direction.normalize();

        let cos_theta = (-unit_dir).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ratio * sin_theta > 1.0;
        let should_reflect = reflectance(cos_theta, ratio) > rand_f64(0.0, 1.0);
        let dir = if cannot_refract || should_reflect {
            reflect(unit_dir, rec.normal)
        } else {
            refract(unit_dir, rec.normal, ratio)
        };
        let ray = Ray::new(rec.position, dir);

        Some(ScatterResult { attenuation, ray })
    }
}
