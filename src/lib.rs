use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x: x, y: y, z: z}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub start: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(start: Point, direction: Point) -> Ray {
        Ray { start: start, direction: direction }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
}

pub struct Scene {
    pub floor: Floor,
    pub light: Point,
    pub spheres: Vec<Sphere>,
    pub eye: Point,
    pub width: u32,
    pub height: u32,
}

impl Sphere {
    pub fn get_intersections(&self, ray: Ray) -> Vec<Point> {
        let x0 = ray.start.x - self.center.x;
        let y0 = ray.start.y - self.center.y;
        let z0 = ray.start.z - self.center.z;
        let a = ray.direction.x.powi(2) + ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * (x0 * ray.direction.x + y0 * ray.direction.y + z0 * ray.direction.z);
        let c = x0.powi(2) + y0.powi(2) + z0.powi(2) - self.radius.powi(2);
        let roots = get_quadratic_equation_roots(a, b, c);
        let mut points = vec![];
        for root in roots {
            if root >= 0.0 {
                let point = Point::new(
                    ray.start.x + root * ray.direction.x,
                    ray.start.y + root * ray.direction.y,
                    ray.start.z + root * ray.direction.z,
                );
                points.push(point)
            }
        }
        return points;
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
                let point = Point::new(
                    ray.start.x + k * ray.direction.x,
                    ray.start.y + k * ray.direction.y,
                    ray.start.z + k * ray.direction.z,
                );
                result.push(point);
            }
        }
        result
    }
}

pub const WHITE: Color = Color { r: 200, g: 200, b: 200 };
pub const BLACK: Color = Color { r: 50, g: 50, b: 50 };
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

pub fn get_brightness(distance_to_light: f32) -> f32 {
    1000.0 / distance_to_light
}

pub fn are_close_points(a: Point, b: Point) -> bool {
    are_close(a.x, b.x) & are_close(a.y, b.y) & are_close(a.z, b.z)
}

pub fn get_quadratic_equation_roots(a: f32, b: f32, c: f32) -> Vec<f32> {
    if a == 0.0 {
        if b == 0.0 {
            panic!("not an equation");
        }
        return vec![-c / b];
    }
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return vec![];
    }
    let d_sqrt = d.sqrt();
    let mut result = vec![];
    result.push((-b + d_sqrt) / (2.0 * a));
    result.push((-b - d_sqrt) / (2.0 * a));
    result
}

pub fn get_closest_point(point: Point, points: &Vec<Point>) -> Option<Point> {
    let mut clone = points.clone();
    clone.sort_by(|a, b| (&get_distance(*a, point)).partial_cmp(&get_distance(*b, point)).unwrap());
    if clone.len() == 0 {
        None
    } else {
        Some(clone[0])
    }
}
