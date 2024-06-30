use glam::DVec3;
use rand::Rng;

#[inline(always)]
pub fn random_vector() -> DVec3 {
    let mut rng = rand::thread_rng();

    DVec3::new(rng.gen(), rng.gen(), rng.gen())
}

#[inline(always)]
pub fn random_vector_range(min: f64, max: f64) -> DVec3 {
    let mut rng = rand::thread_rng();

    DVec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

#[inline(always)]
pub fn random_in_unit_sphere() -> DVec3 {
    loop {
        let vector = random_vector_range(-1., 1.);

        if vector.length_squared() < 1. {
            return vector;
        }
    }
}

#[inline(always)]
pub fn random_in_unit_disk() -> DVec3 {
    loop {
        let mut vector = random_vector_range(-1., 1.);
        vector.z = 0.;
        if vector.length_squared() < 1. {
            return vector;
        }
    }
}

#[inline(always)]
pub fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}

#[inline(always)]
pub fn reflect(v: &DVec3, n: &DVec3) -> DVec3 {
    *v - 2. * v.dot(*n) * *n
}

#[inline(always)]
pub fn refract(uv: &DVec3, n: &DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = (-*uv).dot(*n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = (-(1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;
    r_out_perp + r_out_parallel
}
