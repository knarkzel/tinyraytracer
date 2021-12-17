use std::fs::File;
use std::io::prelude::*;

use fehler::throws;
type Error = anyhow::Error;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

type Vec3f = [f32; 3];

#[throws]
fn main() {
    let mut framebuffer: [Vec3f; WIDTH * HEIGHT] = [Vec3f::default(); WIDTH * HEIGHT];
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            framebuffer[i + j * WIDTH] = [j as f32 / (HEIGHT as f32), i as f32 / (WIDTH as f32), 0.0];
        }
    }

    // Write image
    let mut file = File::create("out.ppm")?;
    file.write_fmt(format_args!("P6\n{} {}\n255\n", WIDTH, HEIGHT))?;
    for i in 0..HEIGHT * WIDTH {
        for value in framebuffer[i] {
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
