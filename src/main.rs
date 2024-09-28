mod math;
mod pixel;

use math::Vec3;
use pixel::Color;

fn main() {
    // Image

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let u = Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0
            );
            let c = Color::from(u);
            
            println!("{c}");
        }
    }
}
