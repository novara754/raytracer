use rand::random;

use crate::vec3::{Color, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Interval(pub f64, pub f64);

impl Interval {
    #[allow(unused)]
    const EMPTY: Self = Interval(f64::INFINITY, f64::NEG_INFINITY);

    #[allow(unused)]
    const UNIVERSE: Self = Interval(f64::NEG_INFINITY, f64::INFINITY);

    #[allow(unused)]
    pub fn contains(&self, value: f64) -> bool {
        self.0 <= value && value <= self.1
    }

    #[allow(unused)]
    pub fn surrounds(&self, value: f64) -> bool {
        self.0 < value && value < self.1
    }

    #[allow(unused)]
    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.0 {
            self.0
        } else if value > self.1 {
            self.1
        } else {
            value
        }
    }
}

pub fn rand_f64(min: f64, max: f64) -> f64 {
    random::<f64>() * (max - min) + max
}

#[allow(unused)]
pub fn rand_vec3(min: f64, max: f64) -> Vec3 {
    Vec3(rand_f64(min, max), rand_f64(min, max), rand_f64(min, max))
}

pub fn rand_unit_vec3() -> Vec3 {
    let theta = rand_f64(0.0, std::f64::consts::PI);
    let phi = rand_f64(0.0, 2.0 * std::f64::consts::PI);

    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();

    Vec3(x, y, z)
}

#[allow(unused)]
pub fn rand_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = rand_unit_vec3();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn linear_to_gamma(color: Color) -> Color {
    Color::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt())
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}

pub fn refract(v: Vec3, normal: Vec3, ratio: f64) -> Vec3 {
    let cos_theta = (-v).dot(normal).min(1.0);
    let r_out_perp = ratio * (v + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn deg2rad(deg: f64) -> f64 {
    deg / 180.0 * std::f64::consts::PI
}
