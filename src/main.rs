#![allow(dead_code, unused)]

use std::fs::File;
use std::io::{stderr, stdout, BufWriter, Write};
use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg};

mod vec3;

fn hello_graphics<W, P>(
    width: u32,
    height: u32,
    mut writer: W,
    mut progress_writer: P,
) -> std::io::Result<()>
where
    W: Write,
    P: Write,
{
    // PPM Header
    // <Header>
    // Every line starts with "P3" in text format
    // Next line has width and height
    // Next line signifies max colour (255 in our case)
    // This max colour value specifies the range as 0..=255
    // </Header>
    // <Body>
    // Now, every triplet after the header is interpreted as a pixel
    // A pixel is written as an RGB triplet and only distinguished by whitespace
    // In pseudo-Haskell,
    // pixels = split whitespace 3 $ words body
    //          where whitespace = r"[ \r\n\t]"
    // The body simply ends, no footer is expected
    // </Body>
    let max_colour = 255;
    writeln!(writer, "P3")?;
    writeln!(writer, "{width} {height}")?;
    writeln!(writer, "{max_colour}")?;
    for j in (0..height).rev() {
        writeln!(progress_writer, "Lines remaining : {}", j)?;
        progress_writer.flush()?;
        for i in 0..width {
            let point = vec3::Color {
                x: (i as f64) / ((width - 1) as f64),
                y: (j as f64) / ((height - 1) as f64),
                z: ((i * i + j * j) as f64).sin() * width as f64,
            };
            writeln!(writer, "{}", point)?;
        }
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    let stdout = stdout().lock();
    let stderr = stderr().lock();
    hello_graphics(256, 256, stdout, stderr)
}
