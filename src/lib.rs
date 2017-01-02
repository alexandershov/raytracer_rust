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

pub struct Ray {
    pub start: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(start: Point, direction: Point) -> Ray {
        Ray { start: start, direction: direction }
    }
}

pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Plane {
        Plane { a: a, b: b, c: c, d: d }
    }

    pub fn get_intersections(&self, ray: Ray) -> Vec<Point> {
        let mut result = vec![];
        let denominator = self.a * ray.direction.x + self.b * ray.direction.y + self.c * ray.direction.z;
        if !are_close(denominator, 0.0) {
            let numerator = -(self.a * ray.start.x + self.b * ray.start.y + self.c * ray.start.z + self.d);
            let k = numerator / denominator;
            if k >= 0.0 {
                let point = Point {
                    x: ray.start.x + k * ray.direction.x,
                    y: ray.start.y + k * ray.direction.y,
                    z: ray.start.z + k * ray.direction.z,
                };
                result.push(point);
            }
        }
        result
    }
}

pub const WHITE: Color = Color { r: 0, g: 0, b: 0 };
pub const BLACK: Color = Color { r: 255, g: 255, b: 255 };
const EPSILON: f32 = 0.001;


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
        if !are_close(point.z, 0.0) {
            panic!("{} is not close to 0.0", point.z);
        }
        let x = (point.x / self.step).floor().abs() as i32;
        let y = (point.y / self.step).floor().abs() as i32;
        if (x % 2) == (y % 2) {
            self.first_color
        } else {
            self.second_color
        }
    }
}

pub fn are_close(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub fn get_distance(a: Point, b: Point) -> f32 {
    let sum = (b.z - a.z).powi(2) + (b.y - a.y).powi(2) + (b.x - a.x).powi(2);
    sum.sqrt()
}

pub fn intensify(color: Color, brightness: f32) -> Color {
    Color {
        r: mul_color_component(color.r, brightness),
        g: mul_color_component(color.g, brightness),
        b: mul_color_component(color.b, brightness),
    }
}


fn mul_color_component(c: u8, brightness: f32) -> u8 {
    let r = ((c as f32) * brightness).min(255.0);
    r as u8
}

