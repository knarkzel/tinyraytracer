use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use fehler::throws;
type Error = anyhow::Error;

use glam::Vec3;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const FOV: f32 = PI / 2.0;

#[derive(Clone)]
struct Material {
    diffuse_color: Vec3,
}

impl Material {
    fn new(diffuse_color: Vec3) -> Self {
        Self { diffuse_color }
    }
}

struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    fn ray_intersect(&self, orig: &Vec3, dir: &Vec3, t0: &mut f32) -> bool {
        let L = self.center - *orig;
        let tca = (L * *dir).length();
        let d2 = (L * L).length() - tca * tca;
        if d2 > self.radius * self.radius {
            return false;
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        *t0 = tca - thc;
        let t1 = tca + thc;
        if *t0 < 0.0 {
            *t0 = t1;
        }
        if *t0 < 0.0 {
            return false;
        }
        return true;
    }
}

fn scene_intersect(
    orig: &Vec3,
    dir: &Vec3,
    spheres: &[Sphere],
    hit: &mut Vec3,
    N: &mut Vec3,
    material: &mut Material,
) -> bool {
    let mut spheres_dist = f32::MAX;
    for sphere in spheres {
        let mut dist_i = 0.0;
        let intersect = sphere.ray_intersect(orig, dir, &mut dist_i);
        if intersect && dist_i < spheres_dist {
            spheres_dist = dist_i;
            *hit = *orig + *dir * dist_i;
            *N = (*hit - sphere.center).normalize();
            *material = sphere.material.clone();
        }
    }
    spheres_dist < 1000.0
}

fn cast_ray(orig: &Vec3, dir: &Vec3, spheres: &[Sphere]) -> Vec3 {
    let (mut point, mut N) = (Vec3::default(), Vec3::default());
    let mut material = Material::new(Vec3::default());
    if scene_intersect(orig, dir, spheres, &mut point, &mut N, &mut material) {
        material.diffuse_color
    } else {
        Vec3::new(0.2, 0.7, 0.8)
    }
}

#[throws]
fn main() {
    let spheres = [
        Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0, Material::new(Vec3::new(0.2, 0.2, 0.2))),
        Sphere::new(Vec3::new(-1.0, 1.5, -12.0), 2.0, Material::new(Vec3::new(0.2, 0.2, 0.2))),
        Sphere::new(Vec3::new(1.5, -0.5, -18.0), 3.0, Material::new(Vec3::new(0.3, 0.3, 0.3))),
        Sphere::new(Vec3::new(7.0, 5.0, -18.0), 4.0, Material::new(Vec3::new(0.4, 0.4, 0.4)))
    ];
    let mut framebuffer = vec![];
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = ((2.0 * (i as f32 + 0.5) / (WIDTH as f32) - 1.0)
                * (FOV / 2.0).tan()
                * WIDTH as f32)
                / HEIGHT as f32;
            let y = (2.0 * (j as f32 + 0.5) / (HEIGHT as f32) - 1.0) * (FOV / 2.0).tan();
            let dir = Vec3::new(x, y, -1.0).normalize();
            framebuffer.push(cast_ray(&Vec3::default(), &dir, &spheres));
        }
    }

    // Write image
    let mut file = File::create("out.ppm")?;
    file.write_fmt(format_args!("P6\n{} {}\n255\n", WIDTH, HEIGHT))?;
    for pixel in framebuffer {
        for color in pixel.to_array().map(|it| (it * 255.0) as u8) {
            file.write(&[color])?;
        }
    }
}
