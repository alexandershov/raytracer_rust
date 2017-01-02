extern crate raytracer;

use raytracer::{BLACK, Floor, Point, WHITE};

#[test]
fn floor_color() {
    let floor = Floor::new(5.0);
    let white_point = Point { x: 0.0, y: 0.0, z: 0.0 };
    assert_eq!(WHITE, floor.color_at(white_point));
    let black_point = Point { x: 5.0, y: 5.0, z: 0.0 };
    assert_eq!(BLACK, floor.color_at(black_point));
}