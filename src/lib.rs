use std::fmt;
use std::ops::{Sub};


pub trait PointInSpace: Copy {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_z(&self) -> f32;
}


pub fn get_closest_point<T: PointInSpace, S: PointInSpace>(point: S, points: &Vec<T>) -> Option<T> {
    let mut clone = points.clone();
    clone.sort_by(|a, b| (&get_distance(*a, point)).partial_cmp(&get_distance(*b, point)).unwrap());
    if clone.len() == 0 {
        None
    } else {
        Some(clone[0])
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x: x, y: y, z: z }
    }
}

impl PointInSpace for Point {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_z(&self) -> f32 {
        self.z
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r={}, g={}, b={})", self.r, self.g, self.b)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ColoredPoint {
    pub point: Point,
    pub color: Color,
}

impl PointInSpace for ColoredPoint {
    fn get_x(&self) -> f32 {
        self.point.x
    }

    fn get_y(&self) -> f32 {
        self.point.y
    }

    fn get_z(&self) -> f32 {
        self.point.z
    }
}

impl ColoredPoint {
    pub fn new(point: Point, color: Color) -> ColoredPoint {
        ColoredPoint { point: point, color: color }
    }
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
    pub sky_color: Color,
    pub spheres: Vec<Sphere>,
    pub eye: Point,
    pub width: u32,
    pub height: u32,
}

impl Scene {
    pub fn color_at(&self, y: u32, z: u32) -> Color {
        let point_at_screen = Point::new(0.0, y as f32, z as f32);
        let ray = Ray::new(self.eye, point_at_screen - self.eye);
        let mut points: Vec<ColoredPoint> = vec![];
        for sphere in self.spheres.iter() {
            for point in sphere.get_colored_intersections(ray) {
                points.push(point);
            }
        }
        let floor_points = self.floor.get_colored_intersections(ray);
        for point in floor_points {
            points.push(point);
        }
        match get_closest_point(self.eye, &points) {
            Some(point) => point.color,
            None => self.sky_color,
        }
    }
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

    pub fn get_colored_intersections(&self, ray: Ray) -> Vec<ColoredPoint> {
        let mut points = vec![];
        for point in self.get_intersections(ray) {
            let colored_point = ColoredPoint::new(point, self.color);
            points.push(colored_point);
        }
        points
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


pub struct Floor {
    step: f32,
    first_color: Color,
    second_color: Color,
    plane: Plane,
}

impl Floor {
    pub fn new(step: f32) -> Floor {
        Floor {
            step: step,
            first_color: BLACK,
            second_color: WHITE,
            plane: Plane { a: 0.0, b: 0.0, c: 1.0, d: 0.0 },
        }
    }

    pub fn get_colored_intersections(&self, ray: Ray) -> Vec<ColoredPoint> {
        let mut result = vec![];
        for point in self.plane.get_intersections(ray) {
            let color = self.color_at(point);
            let colored_point = ColoredPoint::new(point, color);
            result.push(colored_point);
        }
        result
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

pub fn get_distance<S: PointInSpace, T: PointInSpace>(a: S, b: T) -> f32 {
    let sum = (b.get_z() - a.get_z()).powi(2) + (b.get_y() - a.get_y()).powi(2) + (b.get_x() - a.get_x()).powi(2);
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

