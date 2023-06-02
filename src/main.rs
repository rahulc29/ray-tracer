#![allow(dead_code, unused)]

use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use std::cmp::max;
use std::fs::File;
use std::io::{stderr, stdout, BufWriter, Write};
use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg};

mod ray;
mod vec3;

// Currying here might be unnecessary
// Felt right in the moment
fn linear_interpolation(a: Vec3, b: Vec3) -> impl FnOnce(f64) -> Vec3 {
    move |t: f64| (1.0 - t) * a + t * b
}

fn ray_colour(ray: Ray) -> Color {
    let ray_direction_unit = ray.direction.unit_vector();
    let t = 0.5 * (ray_direction_unit.y + 1.0);
    linear_interpolation(
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    )(t)
}

fn render_scene<W, P>(
    width: u32,
    height: u32,
    mut writer: W,
    mut progress_writer: P,
) -> std::io::Result<()>
where
    W: Write,
    P: Write,
{
    // Image attributes
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (aspect_ratio * (image_width as f64)) as i32;
    // Camera attributes
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    // Scene attributes
    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Point3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Point3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Point3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };
    // Write PPM Header
    let max_colour = 255;
    write_ppm_header(&mut writer, width, height, max_colour)?;
    // <Body>
    // Now, every triplet after the header is interpreted as a pixel
    // A pixel is written as an RGB triplet and only distinguished by whitespace
    // In pseudo-Haskell,
    // pixels = split whitespace 3 $ words body
    //          where whitespace = r"[ \r\n\t]"
    // The body simply ends, no footer is expected
    // </Body>
    for j in (0..height).rev() {
        writeln!(progress_writer, "Lines remaining : {}", j)?;
        progress_writer.flush()?;
        for i in 0..width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;
            // ray centered at origin pointing in a linear combination direction parameterised by u and v
            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color = ray_colour(ray);
            writeln!(writer, "{}", pixel_color)?;
        }
    }
    Ok(())
}

// PPM Header
// <Header>
// Every line starts with "P3" in text format
// Next line has width and height
// Next line signifies max colour (255 in our case)
// This max colour value specifies the range as 0..=255
// </Header>
// Borrow checker might be angry
// But we will only use the writer in the main render procedure
// Besides it is an effect handle so it is only gonna be useful when mutable
fn write_ppm_header<W>(
    writer: &mut W,
    width: u32,
    height: u32,
    max_colour: u32,
) -> std::io::Result<()>
where
    W: Write,
{
    writeln!(writer, "P3")?;
    writeln!(writer, "{width} {height}", width = width, height = height)?;
    writeln!(writer, "{max_colour}", max_colour = max_colour)
}

fn main() -> std::io::Result<()> {
    let stdout = stdout().lock();
    let stderr = stderr().lock();
    render_scene(256, 256, stdout, stderr)
}
