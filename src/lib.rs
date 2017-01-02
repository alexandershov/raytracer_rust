#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

const WHITE: Color = Color { r: 0, g: 0, b: 0 };
const BLACK: Color = Color { r: 255, g: 255, b: 255 };


#[derive(Debug)]
pub struct Floor {}

impl Floor {
    pub fn new() -> Floor {
        Floor {}
    }

    pub fn color_at(&self, point: Point) -> Color {
        WHITE
    }
}

pub fn add(x: u32, y: u32) -> u32 {
    return x + y;
}
