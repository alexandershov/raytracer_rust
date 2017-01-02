extern crate raytracer;

use raytracer::{Color, Floor, Point};

#[test]
fn floor_color() {
    let floor = Floor::new();
    let point = Point { x: 0.0, y: 0.0, z: 0.0 };
    let expected_color = Color { r: 0, g: 0, b: 0 };
    assert_eq!(expected_color, floor.color_at(point));
}