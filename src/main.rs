use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};

fn write_color(color: &DVec3) -> String {
    let r = color[0];
    let g = color[1];
    let b = color[2];

    let ir: i64 = (255.999 * r) as i64;
    let ig: i64 = (255.999 * g) as i64;
    let ib: i64 = (255.999 * b) as i64;

    String::from(format!("{ir} {ig} {ib}\n"))
}

struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Ray {
            direction: direction.normalize(),
            origin: origin,
        }
    }

    fn at(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }

    fn ray_color(&self) -> DVec3 {
        let t = hit_sphere(&DVec3::new(0.0,0.0,-1.0), 0.5, self);
        if  t > 0.0 {
            let normal = (self.at(t) - DVec3::new(0.0, 0.0, -1.0)).normalize();
            return 0.5*DVec3::new(normal.x+1.0, normal.y+1.0, normal.z+1.0);
        }

        let unit_direction = self.direction.normalize();
        let a = 0.5*(unit_direction.y+1.0);
        (1.0-a)*DVec3::new(1.0, 1.0, 1.0) + a*DVec3::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &DVec3, radius: f64, ray: &Ray) -> f64 {
    let oc = *center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0*a)
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = {
        let mut temp_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
        temp_height = if temp_height < 1 {1} else {temp_height};
        temp_height
    };

    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ((IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64) as f64);
    let camera_center = DVec3::new(0.0, 0.0, 0.0);

    let viewport_u = DVec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = DVec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    let viewport_upper_left = camera_center
                             - DVec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let file = fs::File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);

    let pixels: String = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .into_iter()
        .progress_count(IMAGE_WIDTH as u64 * IMAGE_HEIGHT as u64)
        .map(|(j, i)| {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray.ray_color();
            write_color(&color)
        })
        .collect();

    writer.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n{pixels}").as_bytes())?;

    Ok(())
}
