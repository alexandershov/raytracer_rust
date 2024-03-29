extern crate raytracer;

use raytracer::{Floor, Point, Plane, Ray, Sphere, Scene, Color, BLACK, WHITE};
use std::f64;

macro_rules! assert_close_colors {
    ($color_a:expr, $color_b:expr, $epsilon:expr) => {{
        assert!(distance_between_colors($color_a, $color_b) < $epsilon, "not close colors {}, {}", $color_a, $color_b);
    }};
}

macro_rules! assert_close_points {
    ($point_a:expr, $point_b:expr, $epsilon:expr) => {{
        assert!(raytracer::get_distance($point_a, $point_b) < $epsilon, "not close points {}, {}", $point_a, $point_b);
    }};
}


#[test]
fn floor_color_at() {
    let floor = Floor::new(5.0, BLACK, WHITE);
    let black_point = Point::new(0.1, 0.1, 0.0);
    assert_eq!(floor.color_at(black_point), BLACK);
    let white_point = Point::new(5.1, 0.1, 0.0);
    assert_eq!(floor.color_at(white_point), WHITE);

    let another_black_point = Point::new(5.1, -0.1, 0.0);
    assert_eq!(floor.color_at(another_black_point), BLACK);
    let another_white_point = Point::new(-5.1, -0.1, 0.0);
    assert_eq!(floor.color_at(another_white_point), WHITE);
}

#[test]
fn ray_plane_intersection() {
    let ray = Ray::new(
        Point::new(-1.0, -1.0, -1.0),
        Point::new(1.0, 1.0, 1.0),
    );
    let plane = Plane::new(0.0, 0.0, 1.0, 0.0);
    let points = plane.get_intersections(ray);
    assert_eq!(1, points.len());
    assert_eq!(points[0], Point::new(0.0, 0.0, 0.0))
}


#[test]
fn ray_plane_no_intersection() {
    let ray = Ray::new(
        Point::new(1.0, 1.0, 1.0),
        Point::new(1.0, 1.0, 1.0));
    let plane = Plane::new(0.0, 0.0, 1.0, 0.0);
    let points = plane.get_intersections(ray);
    assert_eq!(0, points.len());
}

#[test]
fn distance() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let point = Point::new(1.0, 3.0, 5.0);
    let distance = raytracer::get_distance(origin, point);
    assert!(raytracer::are_close(distance, (35.0 as f64).sqrt()));
}

#[test]
fn test_intensify() {
    let color = Color::new(100, 101, 102);
    let intensified = raytracer::intensify(color, 2.0);
    let expected = Color::new(200, 202, 204);
    assert_eq!(intensified, expected);
}


#[test]
fn intensify_very_bright() {
    let color = Color::new(100, 101, 102);
    let intensified = raytracer::intensify(color, 3.0);
    let expected = Color::new(255, 255, 255);
    assert_eq!(intensified, expected);
}

#[test]
fn ray_sphere_intersection() {
    let ray = Ray::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0));
    let sphere = Sphere {
        center: Point::new(0.0, 0.0, 0.0),
        radius: 1.0,
        color: BLACK,
        is_mirror: false,
    };
    let points = sphere.get_intersections(ray);
    assert_eq!(1, points.len());
    assert!(raytracer::are_close_points(points[0], Point::new(1.0, 0.0, 0.0)));
}

#[test]
fn ray_sphere_no_intersection() {
    let ray = Ray::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0));
    let sphere = Sphere {
        center: Point::new(10.0, 10.0, 10.0),
        radius: 1.0,
        color: WHITE,
        is_mirror: false,
    };
    let points = sphere.get_intersections(ray);
    assert_eq!(0, points.len());
}

#[test]
fn quadratic_equation() {
    let solutions = raytracer::get_quadratic_equation_roots(0.0, 2.0, 4.0);
    assert_eq!(solutions, vec![-2.0]);
    let solutions = raytracer::get_quadratic_equation_roots(1.0, 2.0, 1.0);
    assert_eq!(solutions, vec![-1.0, -1.0]);
    let solutions = raytracer::get_quadratic_equation_roots(8.0, 2.0, 1.0);
    assert_eq!(solutions, vec![]);
}

#[test]
fn get_closest_point() {
    let point = Point::new(0.0, 0.0, 0.0);
    let a = Point::new(1.0, 1.0, 1.0);
    let b = Point::new(2.0, 2.0, 2.0);
    match raytracer::get_closest_point(point, &vec![a, b]) {
        Some(actual_point) => assert_eq!(actual_point, a),
        None => assert!(false),
    }
}

#[test]
fn get_no_closest_point() {
    let point = Point::new(0.0, 0.0, 0.0);
    let no_points: Vec<Point> = vec![];
    match raytracer::get_closest_point(point, &no_points) {
        None => assert!(true),
        Some(_) => assert!(false),
    }
}

#[test]
fn screen_color() {
    let green = Color::new(0, 150, 0);
    let sphere = Sphere {
        center: Point::new(-90.0, 10.0, 10.0),
        radius: 10.0,
        color: green,
        is_mirror: false,
    };
    let sky = Color::new(0, 0, 180);
    let scene = Scene {
        floor: Floor::new(32.0, BLACK, WHITE),
        light_source: Point::new(-200.0, 10.0, 200.0),
        sky_color: sky,
        spheres: vec![sphere],
        eye: Point::new(30.0, 30.0, 30.0),
    };
    assert_close_colors!(scene.color_at(255, 255), sky, 0.001);
    // white floor
    // intersection = (-1.034482759, 1.034482759, 0)
    // distance_to_light = 282.2545970402915
    // brightness = 3.542900667
    assert_close_colors!(scene.color_at(2, 1), raytracer::intensify(WHITE, 3.542900667), 0.001);
    // black floor
    // intersection = (-1.034482759, 40.344827586, 0)
    // distance_to_light = 283.7394678436589
    // brightness = 3.524359891
    assert_close_colors!(scene.color_at(40, 1), raytracer::intensify(BLACK, 3.524359891), 0.001);
    // green sphere
    // intersection = (-80.266716, 11.622215, 11.622215)
    // distance_to_light = 223.2148757597421 * 3
    // brightness = 1.493329386
    assert_close_colors!(scene.color_at(25, 25), raytracer::intensify(scene.spheres[0].color, 1.493329386), 0.001);
}

#[test]
fn ray_from_to() {
    let a = Point::new(1.0, 2.0, 3.0);
    let b = Point::new(4.0, 1.0, 8.0);
    let ray = Ray::from_to(a, b);
    assert_eq!(ray.start, a);
    assert_close_points!(ray.direction, Point::new(3.0, -1.0, 5.0), 0.0001);
}

#[test]
fn perpendicular_from_point() {
    let point = Point::new(3.0, 2.0, 0.0);
    let ray = Ray::from_to(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
    let perpendicular_ray = raytracer::get_perpendicular_ray(point, ray).unwrap();
    assert_eq!(perpendicular_ray.start, point);
    assert_close_points!(perpendicular_ray.direction, Point::new(0.0, -2.0, 0.0), 0.001);
}

#[test]
fn sphere_mirroring() {
    let sphere = Sphere {
        center: Point::new(0.0, 0.0, 0.0),
        radius: 1.0,
        color: BLACK,
        is_mirror: true,
    };
    let point = Point::new(10.0, 0.0, 0.0);
    let ray = Ray::from_to(point, Point::new(1.0, 0.0, 0.0));
    let refracted_ray = raytracer::get_refraction_from_sphere(ray, sphere).unwrap();
    assert_close_points!(refracted_ray.start, Point::new(1.0, 0.0, 0.0), 0.001);
    assert_close_points!(refracted_ray.direction, Point::new(9.0, 0.0, 0.0), 0.001);
}

fn distance_between_colors(first: Color, second: Color) -> f64 {
    let sum_squares = (
        (first.r as i32 - second.r as i32).pow(2) +
            (first.g as i32 - second.g as i32).pow(2) +
            (first.b as i32 - second.b as i32).pow(2)) as f64;
    sum_squares.sqrt()
}
