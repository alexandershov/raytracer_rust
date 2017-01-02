#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const WHITE: Color = Color { r: 0, g: 0, b: 0 };
pub const BLACK: Color = Color { r: 255, g: 255, b: 255 };


#[derive(Debug)]
pub struct Floor {
    step: f32,
    first_color: Color,
    second_color: Color,
}

impl Floor {
    pub fn new(step: f32) -> Floor {
        Floor {
            step: step,
            first_color: BLACK,
            second_color: WHITE,
        }
    }

    pub fn color_at(&self, point: Point) -> Color {
        assert!(point.z == 0.0);
        let x = (point.x / self.step).floor() as i32;
        let y = (point.y / self.step).floor() as i32;
        if (x % 2) == (y % 2) {
            self.first_color
        } else {
            self.second_color
        }
    }
}

pub fn add(x: u32, y: u32) -> u32 {
    return x + y;
}
