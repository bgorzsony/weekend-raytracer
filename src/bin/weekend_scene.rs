use glam::DVec3;
use itertools::Itertools;
use raytracer::material::{DielectricMaterial, LambertianMaterial, MetalMaterial};
use rand::Rng;
use raytracer::sphere::Sphere;
use std::path::Path;
use std::{error::Error, sync::Arc};
use raytracer::vectors::{random_vector, random_vector_range};

use raytracer::camera::Camera;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world: Vec<Sphere> = vec![];

    let material_ground = Arc::new(LambertianMaterial {
        albedo: DVec3::new(0.5, 0.5, 0.5),
    });
    world.push(Sphere::new(
        DVec3::new(0., -1000., 0.),
        1000.,
        material_ground,
    ));

    let mut rand = rand::thread_rng();

    let random_spheres = (-11..11)
        .cartesian_product(-11..11)
        .into_iter()
        .map(|(a, b)| {
            let choose_mat: f64 = rand.gen();
            let center = DVec3::new(
                a as f64 + 0.9 * rand.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rand.gen::<f64>(),
            );

            if choose_mat < 0.8 {
                let albedo = random_vector() * random_vector();
                Sphere::new(center, 0.2, Arc::new(LambertianMaterial { albedo: albedo }))
            } else if choose_mat < 0.95 {
                let albedo = random_vector_range(0.5, 1.);
                let fuzz = rand.gen_range(0.0..0.5);

                Sphere::new(center, 0.2, Arc::new(MetalMaterial { albedo, fuzz }))
            } else {
                Sphere::new(
                    center,
                    0.2,
                    Arc::new(DielectricMaterial {
                        refraction_index: 1.5,
                    }),
                )
            }
        });
    world.extend(random_spheres);

    let material1 = Arc::new(DielectricMaterial {
        refraction_index: 1.5,
    });
    world.push(Sphere::new(DVec3::new(0., 1., 0.), 1., material1));

    let material2 = Arc::new(LambertianMaterial {
        albedo: DVec3::new(0.4, 0.2, 0.1),
    });
    world.push(Sphere::new(DVec3::new(-4., 1., 0.), 1., material2));

    let material3 = Arc::new(MetalMaterial {
        albedo: DVec3::new(0.7, 0.6, 0.5),
        fuzz: 0.,
    });
    world.push(Sphere::new(DVec3::new(4., 1., 0.), 1., material3));


    let this_file = file!();
    let with_extension = Path::new(this_file).with_extension("ppm");
    let path = with_extension.file_name().expect("No filename");

    let camera = Camera::init()
        .aspect_ratio(16. / 9.)
        .vup(DVec3::new(0., 1., 0.))
        .look_from(DVec3::new(13., 2., 3.))
        .look_at(DVec3::new(0., 0., -1.))
        .image_width(800)
        .samples_per_pixel(100)
        .max_depth(80)
        .vfov(20.)
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();
    camera.render(path, world)?;

    Ok(())
}
