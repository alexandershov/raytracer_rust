extern crate raytracer;

use raytracer::{BLACK, Floor, Point, WHITE};

#[test]
fn floor_color_at() {
    let floor = Floor::new(5.0);
    let black_point = Point { x: 0.1, y: 0.1, z: 0.0 };
    assert_eq!(BLACK, floor.color_at(black_point));
    let white_point = Point { x: 5.1, y: 0.1, z: 0.0 };
    assert_eq!(WHITE, floor.color_at(white_point));

    let another_black_point = Point { x: 5.1, y: -0.1, z: 0.0 };
    assert_eq!(BLACK, floor.color_at(another_black_point));
    let another_white_point = Point { x: -5.1, y: -0.1, z: 0.0 };
    assert_eq!(WHITE, floor.color_at(another_white_point));
}