extern crate raytracer;

use raytracer::{BLACK, Floor, Point, WHITE, Plane, Ray};

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
    assert_eq!(points[0], Point { x: 0.0, y: 0.0, z: 0.0})
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