extern crate bmp;
extern crate raytracer;

use bmp::{Image, Pixel};
use raytracer::{Floor, Color, Point, Plane, Ray};


fn main() {
    let size = 800;
    let mut image = Image::new(size, size);
    let light_source = Point {
        x: -1000.0,
        y: (size / 2) as f32,
        z: (size / 2) as f32,
    };
    let sphere = raytracer::Sphere {
        center: Point {
            x: -500.0,
            y: (size / 2) as f32,
            z: 50.0,
        },
        radius: 30.0,
    };
    let floor = Floor::new(32.0);
    let floor_plane = Plane::new(0.0, 0.0, 1.0, 0.0);
    let eye = Point {
        x: (size / 2) as f32,
        y: (size / 2) as f32,
        z: (size / 2) as f32
    };
    for y in 0..size {
        for z in 0..size {
            let ray = Ray {
                start: eye,
                direction: Point {
                    x: 0.0 - eye.x,
                    y: (y as f32) - eye.y,
                    z: (z as f32) - eye.z,
                },
            };
            let mut points = floor_plane.get_intersections(ray);
            let mut sphere_points = sphere.get_intersections(ray);
            points.append(&mut sphere_points);
            let color;
            if points.len() == 0 {
                color = Color { r: 0, g: 0, b: 180 }
            } else {
                let point = raytracer::get_closest_point(eye, &points).unwrap();
                let simple_color = floor.color_at(point);
                let distance_to_light = raytracer::get_distance(point, light_source);
                color = raytracer::intensify(simple_color, raytracer::get_brightness(distance_to_light));
            }
            image.set_pixel(size - y - 1, size - z - 1, color_to_pixel(color));
        }
    }
    image.save("/Users/aershov182/tmp/raytracer.bmp").expect("couldn't save image");
}


fn color_to_pixel(color: Color) -> Pixel {
    return Pixel {
        r: color.r,
        g: color.g,
        b: color.b,
    }
}
