extern crate raytracer;

use raytracer::{BLACK, Floor, Point, WHITE, Plane, Ray, are_close, get_distance, Color};
use std::f32;

#[test]
fn floor_color_at() {
    let floor = Floor::new(5.0);
    let black_point = Point { x: 0.1, y: 0.1, z: 0.0 };
    assert_eq!(floor.color_at(black_point), BLACK);
    let white_point = Point { x: 5.1, y: 0.1, z: 0.0 };
    assert_eq!(floor.color_at(white_point), WHITE);

    let another_black_point = Point { x: 5.1, y: -0.1, z: 0.0 };
    assert_eq!(floor.color_at(another_black_point), BLACK);
    let another_white_point = Point { x: -5.1, y: -0.1, z: 0.0 };
    assert_eq!(floor.color_at(another_white_point), WHITE);
}

#[test]
fn ray_plane_intersection() {
    let ray = Ray::new(
        Point { x: -1.0, y: -1.0, z: -1.0 },
        Point { x: 1.0, y: 1.0, z: 1.0 });
    let plane = Plane::new(0.0, 0.0, 1.0, 0.0);
    let points = plane.get_intersections(ray);
    assert_eq!(1, points.len());
    assert_eq!(points[0], Point { x: 0.0, y: 0.0, z: 0.0 })
}


#[test]
fn ray_plane_no_intersection() {
    let ray = Ray::new(
        Point { x: 1.0, y: 1.0, z: 1.0 },
        Point { x: 1.0, y: 1.0, z: 1.0 });
    let plane = Plane::new(0.0, 0.0, 1.0, 0.0);
    let points = plane.get_intersections(ray);
    assert_eq!(0, points.len());
}

#[test]
fn distance() {
    let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
    let point = Point { x: 1.0, y: 3.0, z: 5.0 };
    let distance = get_distance(origin, point);
    assert!(are_close(distance, (35.0 as f32).sqrt()));
}

#[test]
fn intensify() {
    let color = Color { r: 100, g: 101, b: 102 };
    let intensified = raytracer::intensify(color, 2.0);
    let expected = Color { r: 200, g: 202, b: 204 };
    assert_eq!(intensified, expected);
}


#[test]
fn intensify_very_bright() {
    let color = Color { r: 100, g: 101, b: 102 };
    let intensified = raytracer::intensify(color, 3.0);
    let expected = Color { r: 255, g: 255, b: 255 };
    assert_eq!(intensified, expected);
}

#[test]
fn ray_sphere_intersection() {
    let ray = Ray::new(
        Point { x: 0.0, y: 0.0, z: 0.0 },
        Point { x: 1.0, y: 0.0, z: 0.0 });
    let sphere = raytracer::Sphere {
        center: Point { x: 0.0, y: 0.0, z: 0.0 },
        radius: 1.0,
        color: BLACK,
    };
    let points = sphere.get_intersections(ray);
    assert_eq!(1, points.len());
    assert!(raytracer::are_close_points(points[0], Point { x: 1.0, y: 0.0, z: 0.0 }));
}

#[test]
fn ray_sphere_no_intersection() {
    let ray = Ray::new(
        Point { x: 0.0, y: 0.0, z: 0.0 },
        Point { x: 1.0, y: 0.0, z: 0.0 });
    let sphere = raytracer::Sphere {
        center: Point { x: 10.0, y: 10.0, z: 10.0 },
        radius: 1.0,
        color: WHITE,
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
    let point = Point { x: 0.0, y: 0.0, z: 0.0 };
    let a = Point { x: 1.0, y: 1.0, z: 1.0 };
    let b = Point { x: 2.0, y: 2.0, z: 2.0 };
    match raytracer::get_closest_point(point, &vec![a, b]) {
        Some(actual_point) => assert_eq!(actual_point, a),
        None => assert!(false),
    }
}

#[test]
fn get_no_closest_point() {
    let point = Point { x: 0.0, y: 0.0, z: 0.0 };
    match raytracer::get_closest_point(point, &vec![]) {
        None => assert!(true),
        Some(_) => assert!(false),
    }
}

#[test]
fn pixel_color() {
    let green = raytracer::Color { r: 0, g: 150, b: 0 };
    let sphere = raytracer::Sphere {
        center: Point { x: -90.0, y: 10.0, z: 10.0},
        radius: 10.0,
        color: green,
    };
    let scene = raytracer::Scene {
        floor: raytracer::Floor::new(32.0),
        light: raytracer::Point { x: -200.0, y: 10.0, z: 200.0 },
        spheres: vec![sphere],
        eye: Point { x: 30.0, y: 30.0, z: 30.0 },
    };
}
