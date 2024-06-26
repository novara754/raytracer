use std::sync::Arc;

use crate::{
    materials::material::MaterialRef, objects::hittable::HittableList, objects::quad::Quad,
    vec3::Vec3,
};

pub fn cube(a: Vec3, b: Vec3, material: MaterialRef) -> HittableList {
    let min = Vec3(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Vec3(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3(0.0, 0.0, max.z() - min.z());

    HittableList::from_slice(&[
        // front
        Arc::new(Quad::new(Vec3(min.x(), min.y(), max.z()), dx, dy, material)),
        // right
        Arc::new(Quad::new(
            Vec3(max.x(), min.y(), max.z()),
            -dz,
            dy,
            material,
        )),
        // back
        Arc::new(Quad::new(
            Vec3(max.x(), min.y(), min.z()),
            -dx,
            dy,
            material,
        )),
        // left
        Arc::new(Quad::new(Vec3(min.x(), min.y(), min.z()), dz, dy, material)),
        // top
        Arc::new(Quad::new(
            Vec3(min.x(), max.y(), max.z()),
            dx,
            -dz,
            material,
        )),
        // bottom
        Arc::new(Quad::new(Vec3(min.x(), min.y(), min.z()), dx, dz, material)),
    ])
}
