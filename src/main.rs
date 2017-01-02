extern crate bmp;
extern crate raytracer;

use bmp::{Image, Pixel};
use raytracer::{Floor, Color, Point, Plane, Ray};

struct Scene {
    floor: Floor,
    eye: Point,
}

fn main() {
    let size = 200;
    let mut image = Image::new(size, size);
    let floor = Floor::new(16.0);
    let floor_plane = Plane { a: 0.0, b: 0.0, c: 1.0, d: 0.0 };
    let scene = Scene {
        floor: floor,
        eye: Point { x: size / 2, y: size / 2, z: size / 2 },
    };
    for x in 0..size {
        for z in 0..size {
            let ray = Ray {
                start: eye,
                direction: Point {
                    x: x - eye.x,
                    y: 0 - eye.y,
                    z: z - eye.z,
                },
            };
            let points = floor_plane.get_intersections(ray);
            let color;
            if points.len() == 0 {
                color = Color { r: 0, g: 0, b: 180 }
            } else {
                color = floor.color_at(points[0]);
            }
            image.set_pixel(x, z, color_to_pixel(color));
        }
    }
    image.save("/Users/aershov182/tmp/raytracer.bmp").expect("oops");
}


fn color_to_pixel(color: Color) -> Pixel {
    return Pixel {
        r: color.r,
        g: color.g,
        b: color.b,
    }
}
