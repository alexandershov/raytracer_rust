extern crate bmp;
extern crate clap;
extern crate raytracer;

use bmp::{Image, Pixel};
use raytracer::{Floor, Color, Point, Plane, Ray, ColoredPoint, Sphere};


pub struct Scene {
    pub floor: Floor,
    pub light: Point,
    pub sky_color: Color,
    pub spheres: Vec<Sphere>,
    pub eye: Point,
    pub width: u32,
    pub height: u32,
}

fn main() {
    let size = 800;
    let sphere = raytracer::Sphere {
        center: Point::new(
            -500.0,
            (size / 3) as f32,
            80.0,
        ),
        radius: 80.0,
        color: Color::new(0, 180, 0),
    };
    let scene = raytracer::Scene {
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
        width: size,
        height: size,
    };
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
            let point = ColoredPoint { point: p, color: floor.color_at(p) };
            colored_points.push(point);
        },
        _ => (),
    }
    match raytracer::get_closest_point(ray.start, &sphere_points) {
        Some(p) => {
            let point = ColoredPoint { point: p, color: sphere.color };
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
