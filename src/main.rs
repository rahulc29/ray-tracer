use std::io::{Write, stdout};

fn hello_graphics<W>(width: u32, height: u32, mut writer: W) -> std::io::Result<()> where W: Write {
    // PPM Header
    // Every line starts with "P3" in text format
    // Next line has width and height
    // Next line
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?;
    for j in (0..height).rev() {
        for i in 0..width {
            let red = (i as f64) / ((width - 1) as f64);
            let green = (j as f64) / ((height - 1) as f64);
            //let blue = (5f64 * red + 2f64 * green) / 7f64;
            let blue = 0.5f64;
            let intify = |x| (255.999 * x) as u32;
            let red_int = intify(red);
            let green_int = intify(green);
            let blue_int = intify(blue);
            writeln!(writer, "{} {} {}", red_int, green_int, blue_int)?;
        }
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    let stdout = stdout().lock();
    hello_graphics(256, 256, stdout)
}
