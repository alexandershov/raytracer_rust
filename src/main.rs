extern crate bmp;
extern crate raytracer;


use bmp::{Image, Pixel};
use raytracer::{Floor, Color, Point};

fn main() {
    let mut image = Image::new(256, 256);
    draw_floor(&mut image);
    image.save("/Users/aershov182/tmp/raytracer.bmp").expect("oops");
}

fn draw_floor(image: &mut Image) {
    let floor = Floor::new(16.0);
    for x in 0..255 {
        for y in 0..255 {
            let point = Point { x: x as f32, y: y as f32, z: 0.0 };
            let color = floor.color_at(point);
            image.set_pixel(x, y, color_to_pixel(color));
        }
    }
}

fn color_to_pixel(color: Color) -> Pixel {
    return Pixel {
        r: color.r,
        g: color.g,
        b: color.b,
    }
}
