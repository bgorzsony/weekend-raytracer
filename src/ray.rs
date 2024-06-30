use glam::DVec3;

use crate::hittable::Hittable;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Ray {
            direction: direction.normalize(),
            origin: origin,
        }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }

    pub fn ray_color<T: Hittable + Sync>(&self, depth: i64, world: &Vec<T>) -> DVec3 {
        if depth <= 0 {
            return DVec3::ZERO;
        }

        if let Some(hit) = world.hit(self, &(0.0001..f64::INFINITY)) {
            if let Some(scatter) = hit.material.scatter(self, &hit) {
                return scatter.color * scatter.ray.ray_color(depth - 1, world);
            } else {
                return DVec3::ZERO;
            }
        } else {
            let unit_direction = self.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
        }
    }
}
