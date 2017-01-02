extern crate bmp;
extern crate raytracer;

use bmp::{Image, Pixel};
use raytracer::{Floor, Color, Point, Plane, Ray};


struct ColoredPoint {
    point: Point,
    color: Color,
}

const SPHERE_COLOR: Color = Color { r: 0, g: 180, b: 0};



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
            z: 30.0,
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
            let color;
            let colored_points = get_colored_points(&floor, &floor_plane, &sphere, ray);
            if colored_points.len() == 0 {
                color = Color { r: 0, g: 0, b: 180 }
            } else {
                let point = colored_points[0].point;
                let simple_color = colored_points[0].color;
                let distance_to_light = raytracer::get_distance(point, light_source);
                color = raytracer::intensify(simple_color, raytracer::get_brightness(distance_to_light));
            }
            image.set_pixel(size - y - 1, size - z - 1, color_to_pixel(color));
        }
    }
    image.save("/Users/aershov182/tmp/raytracer.bmp").expect("couldn't save image");
}

fn get_colored_points(floor: &Floor, floor_plane: &Plane, sphere: &raytracer::Sphere, ray: Ray) -> Vec<ColoredPoint> {
    let floor_points = floor_plane.get_intersections(ray);
    let sphere_points = sphere.get_intersections(ray);
    let mut colored_points = vec![];
    match raytracer::get_closest_point(ray.start, &floor_points) {
        Some(p) => {
            let point = ColoredPoint {point: p, color: floor.color_at(p)};
            colored_points.push(point);
        },
        _ => (),
    }
    match raytracer::get_closest_point(ray.start, &sphere_points) {
        Some(p) => {
            let point = ColoredPoint {point: p, color: SPHERE_COLOR};
            if colored_points.len() != 0 {
                if raytracer::get_distance(p, ray.start) < raytracer::get_distance(colored_points[0].point, ray.start) {
                    colored_points.pop();
                    colored_points.push(point)
                }
            }
        },
        _ => (),
    }
    return colored_points;
}


fn color_to_pixel(color: Color) -> Pixel {
    return Pixel {
        r: color.r,
        g: color.g,
        b: color.b,
    }
}
