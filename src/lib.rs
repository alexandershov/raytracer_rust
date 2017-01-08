use std::fmt;
use std::ops::{Add, Mul, Sub};


pub trait PointInSpace {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_z(&self) -> f64;
}


pub fn get_closest_point<T, S>(point: S, points: &Vec<T>) -> Option<T> where T: PointInSpace + Copy, S: PointInSpace + Copy {
    let mut clone = points.clone();
    clone.sort_by(|a, b| (&get_distance(*a, point)).partial_cmp(&get_distance(*b, point)).unwrap());
    if clone.len() == 0 {
        None
    } else {
        Some(clone[0])
    }
}

pub fn get_distance<S, T>(a: S, b: T) -> f64 where S: PointInSpace, T: PointInSpace {
    let sum = (b.get_x() - a.get_x()).powi(2) + (b.get_y() - a.get_y()).powi(2) + (b.get_z() - a.get_z()).powi(2);
    sum.sqrt()
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x: x, y: y, z: z }
    }
}

impl PointInSpace for Point {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_z(&self) -> f64 {
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

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scale: f64) -> Point {
        Point::new(self.x * scale, self.y * scale, self.z * scale)
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
    fn get_x(&self) -> f64 {
        self.point.x
    }

    fn get_y(&self) -> f64 {
        self.point.y
    }

    fn get_z(&self) -> f64 {
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

    pub fn from_to(start: Point, end: Point) -> Ray {
        Ray::new(start, end - start)
    }
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub is_mirror: bool,
}

pub struct Scene {
    pub floor: Floor,
    pub light_source: Point,
    pub sky_color: Color,
    pub spheres: Vec<Sphere>,
    pub eye: Point,
}

impl Scene {
    pub fn color_at(&self, y: u32, z: u32) -> Color {
        let point_at_screen = Point::new(0.0, y as f64, z as f64);
        let ray = Ray::from_to(self.eye, point_at_screen);
        let (points, cur_ray) = self.get_all_colored_intersections(ray, true);
        let closest_point = get_closest_point(cur_ray.start, &points);
        let lighted_point = closest_point.map(|p| self.apply_lightning(p));
        match lighted_point {
            Some(point) => point.color,
            None => self.sky_color,
        }
    }

    fn apply_lightning(&self, point: ColoredPoint) -> ColoredPoint {
        let ray_to_light = Ray::from_to(point.point, self.light_source);
        let (points, _) = self.get_all_colored_intersections(ray_to_light, false);
        let obstacle_point = get_closest_point(point, &exclude_close_points(point, &points));
        let coeff = match obstacle_point {
            Some(_) => 3.0,  // shadow
            None => 1.0,
        };
        let distance_to_light = get_distance(point, self.light_source) * coeff;
        ColoredPoint::new(
            point.point,
            intensify(point.color, get_brightness(distance_to_light)),
        )
    }

    fn get_all_colored_intersections(&self, ray: Ray, with_mirroring: bool) -> (Vec<ColoredPoint>, Ray) {
        let mut cur_ray = ray;
        let mut num_iter = 0;
        if with_mirroring {
            num_iter = 3;
        }
        let floor_points = get_closest_point(ray.start, &self.floor.get_colored_intersections(ray));
        let sphere_points = get_closest_point(ray.start, &self.get_sphere_intersections(ray));
        if with_mirroring & floor_points.is_some() & !sphere_points.is_some() {
            return (vec![floor_points.unwrap()], ray);
        }
        for _ in 1..(num_iter + 1) {
            for sphere in self.spheres.iter() {
                cur_ray = get_refraction_from_sphere(cur_ray, *sphere).unwrap_or(cur_ray);
            }
        }
        let mut points = self.floor.get_colored_intersections(cur_ray);
        points.extend(self.get_sphere_intersections(cur_ray));
        (points, cur_ray)
    }

    fn get_sphere_intersections(&self, ray: Ray) -> Vec<ColoredPoint> {
        let mut points = vec![];
        for sphere in self.spheres.iter() {
            points.extend(sphere.get_colored_intersections(ray));
        }
        points
    }
}

fn exclude_close_points<S, T>(point: S, points: &Vec<T>) -> Vec<T> where S: PointInSpace + Copy, T: PointInSpace + Copy {
    let mut result = vec![];
    for item in points {
        if !are_close_points(point, *item) {
            result.push(*item);
        }
    }
    result
}

impl Sphere {
    pub fn get_intersections(&self, ray: Ray) -> Vec<Point> {
        let p = ray.start - self.center;
        let a = ray.direction.x.powi(2) + ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * (p.x * ray.direction.x + p.y * ray.direction.y + p.z * ray.direction.z);
        let c = p.x.powi(2) + p.y.powi(2) + p.z.powi(2) - self.radius.powi(2);
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
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Plane {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Plane {
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
const EPSILON: f64 = 0.001;


pub struct Floor {
    step: f64,
    first_color: Color,
    second_color: Color,
    plane: Plane,
}

impl Floor {
    pub fn new(step: f64, first_color: Color, second_color: Color) -> Floor {
        Floor {
            step: step,
            first_color: first_color,
            second_color: second_color,
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

pub fn are_close(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn intensify(color: Color, brightness: f64) -> Color {
    Color {
        r: multiply_color_component(color.r, brightness),
        g: multiply_color_component(color.g, brightness),
        b: multiply_color_component(color.b, brightness),
    }
}


fn multiply_color_component(c: u8, brightness: f64) -> u8 {
    let r = ((c as f64) * brightness).min(255.0);
    r as u8
}

pub fn get_brightness(distance_to_light: f64) -> f64 {
    1000.0 / distance_to_light
}

pub fn are_close_points<S, T>(a: T, b: S) -> bool where S: PointInSpace, T: PointInSpace {
    are_close(a.get_x(), b.get_x()) & are_close(a.get_y(), b.get_y()) & are_close(a.get_z(), b.get_z())
}

pub fn get_quadratic_equation_roots(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a == 0.0 {
        if b == 0.0 {
            // TODO: wtf?
            return vec![];
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

pub fn get_perpendicular_ray(point: Point, ray: Ray) -> Option<Ray> {
    let numerator = (point.x - ray.start.x) * ray.direction.x + (point.y - ray.start.y) * ray.direction.y + (point.z - ray.start.z) * ray.direction.z;
    let denominator = ray.direction.x.powi(2) + ray.direction.y.powi(2) + ray.direction.z.powi(2);
    if are_close(denominator, 0.0) {
        return None
    }
    let k = numerator / denominator;
    if k < 0.0 {
        return None
    }
    //    panic!("intersection inter = {}, perp.start = {}, perp.direction = {}, k = {}", point, ray.start, ray.direction, k);
    let point_on_ray = Point::new(
        ray.start.x + k * ray.direction.x,
        ray.start.y + k * ray.direction.y,
        ray.start.z + k * ray.direction.z,
    );
    Some(Ray::from_to(point, point_on_ray))
}

pub fn get_refraction_from_sphere(ray: Ray, sphere: Sphere) -> Option<Ray> {
    let intersections = sphere.get_intersections(ray);
    let closest_point = get_closest_point(ray.start, &intersections);
    if !closest_point.is_some() {
        return None
    }
    let point = closest_point.unwrap();
    if !sphere.is_mirror {
        return Some(Ray::from_to(ray.start, point));
    }
    let perpendicular_ray = get_perpendicular_ray(ray.start, Ray::from_to(sphere.center, point));
    perpendicular_ray
        .map(|r| r.start + r.direction * 2.0)
        .map(|p| Ray::from_to(point, p))
}

