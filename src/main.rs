use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use fehler::throws;
type Error = anyhow::Error;

use glam::Vec3;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const FOV: f32 = PI / 2.0;

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    fn ray_intersect(&self, orig: &Vec3, dir: &Vec3, t0: &mut f32) -> bool {
        let L = self.center - *orig;
        let tca = L * *dir;
        let d2 = L * L - tca * tca;
        if d2.length() > self.radius * self.radius { return false; }
        let thc = (self.radius * self.radius - d2.length()).sqrt();
        *t0 = (tca - thc).length();
        let t1 = tca + thc;
        if *t0 < 0.0 { *t0 = t1.length(); }
        if *t0 < 0.0 { return false; }
        return true;
    }
}

fn cast_ray(orig: &Vec3, dir: &Vec3, sphere: &Sphere) -> Vec3 {
    let mut sphere_dist = f32::MAX;
    if !sphere.ray_intersect(orig, dir, &mut sphere_dist) {
        Vec3::new(0.2, 0.7, 0.8)
    } else {
        Vec3::new(0.4, 0.4, 0.3)
    }
}

#[throws]
fn main() {
    let mut framebuffer = vec![];
    let sphere = Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 4.0);
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2.0 * (i as f32 + 0.5) / (WIDTH as f32) - 1.0) * (FOV / 2.0).tan() * WIDTH as f32 / HEIGHT as f32;
            let y = (2.0 * (j as f32 + 0.5) / (HEIGHT as f32) - 1.0) * (FOV / 2.0).tan();
            let dir = Vec3::new(x, y, -1.0).normalize();
            framebuffer.push(cast_ray(&Vec3::default(), &dir, &sphere));
        }
    }

    // Write image
    let mut file = File::create("out.ppm")?;
    file.write_fmt(format_args!("P6\n{} {}\n255\n", WIDTH, HEIGHT))?;
    for pixel in framebuffer {
        for value in pixel.to_array() {
            let byte = if value > 1.0 {
                255.0
            } else if value < 0.0 {
                0.0
            } else {
                255.0 * value
            };
            file.write(&[byte as u8])?;
        }
    }
}
