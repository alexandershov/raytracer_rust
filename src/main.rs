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
            y: (size / 3) as f32,
            z: 80.0,
        },
        radius: 80.0,
    };
    let floor = Floor::new(64.0);
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
            let colored_points = get_colored_points(&floor, &floor_plane, &sphere, ray, false);
            if colored_points.len() == 0 {
                color = Color { r: 0, g: 0, b: 180 }
            } else {
                let point = colored_points[0].point;
                let simple_color = colored_points[0].color;
                let ray_to_light = raytracer::Ray {
                    start: point,
                    direction: raytracer::Point {
                        x: light_source.x - point.x,
                        y: light_source.y - point.y,
                        z: light_source.z - point.z,
                    },
                };
                let distance_to_light;
                let points_to_light = get_colored_points(&floor, &floor_plane, &sphere, ray_to_light, true);
                if points_to_light.len() != 0 {
                    distance_to_light = raytracer::get_distance(point, light_source) * 3.0;
                } else {
                    distance_to_light = raytracer::get_distance(point, light_source);
                }
                color = raytracer::intensify(simple_color, raytracer::get_brightness(distance_to_light));
            }
            image.set_pixel(size - y - 1, size - z - 1, color_to_pixel(color));
        }
    }
    image.save("/Users/aershov182/tmp/raytracer.bmp").expect("couldn't save image");
}

fn get_colored_points(floor: &Floor, floor_plane: &Plane, sphere: &raytracer::Sphere, ray: Ray,
        exclude_ray_start: bool) -> Vec<ColoredPoint> {
    let mut floor_points = floor_plane.get_intersections(ray);
    let mut sphere_points = sphere.get_intersections(ray);
    if exclude_ray_start {
        floor_points = exclude_close_points(ray.start, &floor_points);
        sphere_points = exclude_close_points(ray.start, &sphere_points);
    }
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
            } else {
                colored_points.push(point)
            }
        },
        _ => (),
    }
    return colored_points;
}


fn exclude_close_points(point: raytracer::Point, points: &Vec<raytracer::Point>) -> Vec<raytracer::Point> {
    let mut result = vec![];
    for item in points {
        if !raytracer::are_close_points(point, *item) {
            result.push(*item);
        }
    }
    result
}


fn color_to_pixel(color: Color) -> Pixel {
    return Pixel {
        r: color.r,
        g: color.g,
        b: color.b,
    }
}
