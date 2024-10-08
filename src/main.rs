mod math;
mod pixel;
mod render;

use math::{Point3, Vec3};
use pixel::Color;
use render::Ray;

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(&ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    let c = (1.0 - a) * Point3::ones() + a * Point3::new(0.5, 0.7, 1.0);

    Color::from(c)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera

    let camera_center = Point3::zeros();
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    // Calculate the vectors across the horiaontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let c = ray_color(&ray);

            println!("{c}");
        }
    }
}
