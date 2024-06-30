use std::ops::Neg;

use glam::DVec3;
use rand::Rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vectors::{random_unit_vector, reflect, refract},
};

pub struct Scattered {
    pub color: DVec3,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<Scattered>;
}

pub struct LambertianMaterial {
    pub albedo: DVec3,
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<Scattered> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        if scatter_direction.abs_diff_eq(DVec3::ZERO, 1e-8) {
            scatter_direction = hit.normal;
        }

        Some(Scattered {
            color: self.albedo,
            ray: Ray::new(hit.point, scatter_direction),
        })
    }
}

pub struct MetalMaterial {
    pub albedo: DVec3,
    pub fuzz: f64,
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<Scattered> {
        let mut reflected = reflect(&ray_in.direction, &hit.normal);
        reflected = reflected.normalize() + (self.fuzz * random_unit_vector());
        let scattered_ray = Ray::new(hit.point, reflected);

        if scattered_ray.direction.dot(hit.normal) > 0. {
            return Some(Scattered {
                color: self.albedo,
                ray: scattered_ray,
            });
        } else {
            return None;
        }
    }
}

pub struct DielectricMaterial {
    pub refraction_index: f64,
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<Scattered> {
        let mut rand = rand::thread_rng();
        let ri = if hit.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = ray_in.direction.normalize().neg().dot(hit.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.;

        let direction = if cannot_refract || reflectance(cos_theta, ri) > rand.gen::<f64>() {
            reflect(&ray_in.direction.normalize(), &hit.normal)
        } else {
            refract(&ray_in.direction.normalize(), &hit.normal, ri)
        };

        return Some(Scattered {
            color: DVec3::new(1., 1., 1.),
            ray: Ray::new(hit.point, direction),
        });
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powf(5.);
}
