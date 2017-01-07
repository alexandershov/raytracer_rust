extern crate bmp;
extern crate clap;
extern crate raytracer;

use bmp::{Image, Pixel};
use raytracer::{Scene, Sphere, Point, Color, Floor};


fn main() {
    let size = 800;
    let scene = make_scene(size);
    let mut image = Image::new(size, size);

    for y in 0..size {
        for z in 0..size {
            let color = scene.color_at(y, z);
            image.set_pixel(size - 1 - y, size - 1 - z, color_to_pixel(color));
        }
    }
    let matches = clap::App::new("raytracer")
        .version("0.1.0")
        .args_from_usage(
            "<OUTPUT_PATH> 'output .bmp file path'"
        ).get_matches();
    let path = matches.value_of("OUTPUT_PATH").unwrap();
    image.save(path).expect("couldn't save image");
}

fn make_scene(size: u32) -> Scene {
    let sphere = Sphere {
        center: Point::new(
            -500.0,
            (size / 3) as f32,
            80.0,
        ),
        radius: 80.0,
        color: Color::new(0, 180, 0),
    };
    Scene {
        floor: Floor::new(64.0),
        light: Point::new(
            -1000.0,
            (size / 2) as f32,
            (size / 2) as f32,
        ),
        sky_color: Color::new(0, 0, 180),
        spheres: vec![sphere],
        eye: Point::new(
            (size / 2) as f32,
            (size / 2) as f32,
            (size / 2) as f32
        ),
    }
}


fn color_to_pixel(color: Color) -> Pixel {
    return Pixel::new(color.r, color.g, color.b);
}
