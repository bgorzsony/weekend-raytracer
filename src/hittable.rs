use std::ops::Range;

use glam::DVec3;

use crate::material::Materials;

use super::ray::Ray;

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Materials,
}

impl HitRecord {
    pub fn new(point: DVec3, normal: DVec3, t: f64, material: Materials) -> Self {
        Self {
            point: point,
            normal: normal,
            t: t,
            front_face: false,
            material: material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &DVec3) {
        self.front_face = ray.direction.dot(*outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Range<f64>) -> Option<HitRecord>;
}

impl<T> Hittable for Vec<T>
where
    T: Hittable + Sync,
{
    fn hit(&self, ray: &Ray, interval: &Range<f64>) -> Option<HitRecord> {
        let (_last_hit, hit) = self.iter().fold(
            (interval.end, None),
            |acc: (f64, Option<HitRecord>), obj| {
                if let Some(temp_hit) = obj.hit(ray, &(interval.start..acc.0)) {
                    (temp_hit.t, Some(temp_hit))
                } else {
                    acc
                }
            },
        );

        return hit;
    }
}
