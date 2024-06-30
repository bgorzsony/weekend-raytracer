use std::{fs, path::Path};

use glam::DVec3;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;

use crate::{hittable::Hittable, ray::Ray, vectors::random_in_unit_disk};

pub struct CameraBuilder {
    image_width: i64,
    aspect_ratio: f64,
    samples_per_pixel: i64,
    max_depth: i64,
    vfov: f64,
    look_from: DVec3,
    look_at: DVec3,
    vup: DVec3,
    defocus_angle: f64,
    focus_dist: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 16. / 9.,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.,
            look_from: DVec3::ZERO,
            look_at: DVec3::new(0., 0., -1.),
            vup: DVec3::new(0., 1., 0.),
            defocus_angle: 0.,
            focus_dist: 10.,
        }
    }
}

impl CameraBuilder {
    pub fn image_width(mut self, image_width: i64) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn samples_per_pixel(mut self, samples: i64) -> Self {
        self.samples_per_pixel = samples;
        self
    }

    pub fn max_depth(mut self, depth: i64) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn look_from(mut self, from: DVec3) -> Self {
        self.look_from = from;
        self
    }

    pub fn look_at(mut self, look_at: DVec3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn vup(mut self, vup: DVec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, angle: f64) -> Self {
        self.defocus_angle = angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn build(&mut self) -> Camera {
        let image_height = ((self.image_width as f64 / self.aspect_ratio) as i64).max(1);

        let theta = self.vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * self.focus_dist;

        let viewport_width = viewport_height * ((self.image_width as f64) / image_height as f64);

        let w = (self.look_from - self.look_at).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            self.look_from - (self.focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width: self.image_width,
            image_height: image_height,
            center: self.look_from,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            pixel00_loc: pixel00_loc,
            samples_per_pixel: self.samples_per_pixel,
            pixel_samples_scale: 1. / self.samples_per_pixel as f64,
            max_depth: self.max_depth,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle: self.defocus_angle,
        }
    }
}

pub struct Camera {
    image_width: i64,
    image_height: i64,

    center: DVec3,

    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,

    pixel00_loc: DVec3,

    samples_per_pixel: i64,
    pixel_samples_scale: f64,

    max_depth: i64,

    defocus_angle: f64,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
}

impl Camera {
    pub fn init() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn render<T: Hittable + Sync, P: AsRef<Path>>(&self, filepath: P, world: Vec<T>) -> std::io::Result<()> {
        let pixels: String = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(i64, i64)>>()
            .into_par_iter()
            .progress_count(self.image_width as u64 * self.image_height as u64)
            .map(|(j, i)| {
                let sampled_color =
                    (0..self.samples_per_pixel)
                        .into_iter()
                        .fold(DVec3::ZERO, |acc, _| {
                            let ray = self.get_ray(i, j);
                            acc + ray.ray_color(self.max_depth, &world)
                        })
                        * self.pixel_samples_scale;

                write_color(&sampled_color)
            })
            .collect();

        fs::write(
            filepath,
            format!(
                "P3\n{} {}\n255\n{}",
                self.image_width, self.image_height, pixels
            ),
        )
    }

    fn get_ray(&self, i: i64, j: i64) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> DVec3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

fn write_color(color: &DVec3) -> String {
    let r = linear_to_gamma(color[0]);
    let g = linear_to_gamma(color[1]);
    let b = linear_to_gamma(color[2]);

    let ir: i64 = (255.999 * r) as i64;
    let ig: i64 = (255.999 * g) as i64;
    let ib: i64 = (255.999 * b) as i64;

    String::from(format!("{ir} {ig} {ib}\n"))
}

fn sample_square() -> DVec3 {
    let mut rand = rand::thread_rng();

    DVec3::new(rand.gen::<f64>() - 0.5, rand.gen::<f64>() - 0.5, 0.)
}

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0. {
        linear.sqrt()
    } else {
        0.
    }
}
