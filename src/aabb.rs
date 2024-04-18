use crate::{ray::Ray, util::Interval, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    #[allow(unused)]
    pub fn empty() -> Self {
        Self {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }

    #[allow(unused)]
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut s = Self { x, y, z };
        s.ensure_minimum_extents();
        s
    }

    pub fn span_points(a: Vec3, b: Vec3) -> Self {
        let mut s = Self {
            x: Interval(a.x().min(b.x()), a.x().max(b.x())),
            y: Interval(a.y().min(b.y()), a.y().max(b.y())),
            z: Interval(a.z().min(b.z()), a.z().max(b.z())),
        };
        s.ensure_minimum_extents();
        s
    }

    pub fn combine(a: Aabb, b: Aabb) -> Self {
        Self {
            x: Interval::combine(a.x, b.x),
            y: Interval::combine(a.y, b.y),
            z: Interval::combine(a.z, b.z),
        }
    }

    pub fn axis(&self, i: usize) -> Interval {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("invalid AABB axis index"),
        }
    }

    pub fn hit(&self, ray: &Ray, mut allowed_t: Interval) -> bool {
        for i in 0..3 {
            let t0 = (self.axis(i).0 - ray.origin[i]) / ray.direction[i];
            let t1 = (self.axis(i).1 - ray.origin[i]) / ray.direction[i];

            let t_min = t0.min(t1);
            let t_max = t0.max(t1);

            allowed_t.0 = t_min.max(allowed_t.0);
            allowed_t.1 = t_max.min(allowed_t.1);

            if allowed_t.1 <= allowed_t.0 {
                return false;
            }
        }

        true
    }

    fn ensure_minimum_extents(&mut self) {
        let minimum = 0.0001;
        if self.x.size() < minimum {
            self.x.expand(minimum);
        }
        if self.y.size() < minimum {
            self.y.expand(minimum);
        }
        if self.z.size() < minimum {
            self.z.expand(minimum);
        }
    }
}
