use std::ops::Range;

use glam::DVec3;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Materials,
    ray::Ray,
};

pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Materials,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Materials) -> Self {
        Self {
            center: center,
            radius: radius.max(0.),
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Range<f64>) -> Option<crate::hittable::HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length().powf(2.);
        let h = ray.direction.dot(oc);
        let c = oc.length().powf(2.) - self.radius.powf(2.);

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.contains(&root) {
            root = (h + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let mut hit = HitRecord::new(point, normal, root, self.material.clone());
        hit.set_face_normal(ray, &normal);
        return Some(hit);
    }
}
